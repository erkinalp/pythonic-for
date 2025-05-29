use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse_macro_input, Block, ExprBreak, visit_mut::{self, VisitMut}, Token, Ident, Expr, Result,
    parse::{Parse, ParseStream}, RangeLimits, Pat, Lifetime, Lit, ExprLit,
};

struct TransformBodyInput {
    label: Lifetime,
    _comma: Token![,],
    body: Block,
}

impl Parse for TransformBodyInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            label: input.parse()?,
            _comma: input.parse()?,
            body: input.parse()?,
        })
    }
}

struct BreakTransformer {
    in_pythonic_loop_body: bool, 
    current_loop_label: Lifetime,
}

impl BreakTransformer {
    fn new(loop_label: Lifetime) -> Self {
        Self {
            in_pythonic_loop_body: true, 
            current_loop_label: loop_label,
        }
    }
}

impl VisitMut for BreakTransformer {
    fn visit_expr_for_loop_mut(&mut self, node: &mut syn::ExprForLoop) {
        let originally_in_pythonic_loop_body = self.in_pythonic_loop_body;
        self.in_pythonic_loop_body = false; 
        visit_mut::visit_expr_for_loop_mut(self, node);
        self.in_pythonic_loop_body = originally_in_pythonic_loop_body; 
    }

    fn visit_expr_while_mut(&mut self, node: &mut syn::ExprWhile) {
        let originally_in_pythonic_loop_body = self.in_pythonic_loop_body;
        self.in_pythonic_loop_body = false;
        visit_mut::visit_expr_while_mut(self, node);
        self.in_pythonic_loop_body = originally_in_pythonic_loop_body;
    }

    fn visit_expr_loop_mut(&mut self, node: &mut syn::ExprLoop) {
        let originally_in_pythonic_loop_body = self.in_pythonic_loop_body;
        self.in_pythonic_loop_body = false;
        visit_mut::visit_expr_loop_mut(self, node);
        self.in_pythonic_loop_body = originally_in_pythonic_loop_body;
    }
    
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Break(expr_break) = node {
            if self.in_pythonic_loop_body {
                let user_label = &expr_break.label;
                let break_expr_val = &expr_break.expr;

                let target_label_to_quote = match user_label {
                    Some(l) => quote!(#l),
                    None => {
                        let l = &self.current_loop_label;
                        quote!(#l)
                    }
                };
            
                let new_expr_tokens = if let Some(inner_expr) = break_expr_val {
                    quote! {{
                        _break_occurred = true;
                        break #target_label_to_quote #inner_expr;
                    }}
                } else {
                    quote! {{
                        _break_occurred = true;
                        break #target_label_to_quote;
                    }}
                };
                
                match syn::parse2::<Expr>(new_expr_tokens) {
                    Ok(new_expr_block) => {
                        *node = new_expr_block; 
                        return; 
                    }
                    Err(e) => {
                        let error_ts = syn::Error::new_spanned(
                            node, // Span of the original break expression node
                            format!("Internal error in pythonic-for: Failed to construct break transformation: {}", e)
                        ).to_compile_error();
                        // Try to parse the compile_error! invocation itself as an Expr
                        match syn::parse2::<Expr>(error_ts) {
                           Ok(err_expr) => *node = err_expr, // Replace node with the compile_error! expression
                           Err(_) => { /* Last resort: if parsing compile_error! itself fails, leave node unchanged or panic */ }
                        }
                        return;
                    }
                }
            }
        }
        // Default visitation for other expressions or if not transformed
        visit_mut::visit_expr_mut(self, node);
    }
}


#[proc_macro]
pub fn transform_body(input: TokenStream) -> TokenStream {
    // This parses `label, { body }`
    let parsed = match syn::parse::<TransformBodyInput>(input.clone()) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };
    let mut body_block = parsed.body;
    let loop_label = parsed.label;
    
    let mut transformer = BreakTransformer::new(loop_label);
    transformer.visit_block_mut(&mut body_block);
    
    let expanded = quote! {
        #body_block
    };
    
    expanded.into()
}

mod kw {
    syn::custom_keyword!(in_kw);
    syn::custom_keyword!(else_kw);
    syn::custom_keyword!(step);
}

struct StepClause {
    step_kw: kw::step,
    eq_token: Token![=],
    step_expr: Expr,
}

impl Parse for StepClause {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            step_kw: input.parse()?,
            eq_token: input.parse()?,
            step_expr: input.parse()?,
        })
    }
}

enum Iterable {
    Expr(Expr), 
    RangeWithStep {
        range_expr: Expr, 
        comma_token: Token![,],
        step_clause: StepClause,
    },
}

impl Parse for Iterable {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr: Expr = input.parse()?;
        if input.peek(Token![,]) && input.peek2(kw::step) {
            Ok(Iterable::RangeWithStep {
                range_expr: expr, 
                comma_token: input.parse()?,
                step_clause: input.parse()?,
            })
        } else {
            Ok(Iterable::Expr(expr))
        }
    }
}

struct ElseClause {
    else_kw: kw::else_kw,
    else_body: Block,
}

impl Parse for ElseClause {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ElseClause {
            else_kw: input.parse()?,
            else_body: input.parse()?,
        })
    }
}

struct PythonicForInput {
    var: Ident,
    in_kw: kw::in_kw,
    iterable: Iterable,
    body: Block,
    else_clause: Option<ElseClause>,
}

impl Parse for PythonicForInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let var_pat: Pat = input.parse()?;
        let var = match var_pat {
            Pat::Ident(pat_ident) => {
                if !pat_ident.attrs.is_empty() || pat_ident.by_ref.is_some() || pat_ident.mutability.is_some() || pat_ident.subpat.is_some() {
                     return Err(syn::Error::new_spanned(pat_ident, "Loop variable must be a simple identifier (e.g., `i`), not a complex pattern."));
                }
                pat_ident.ident
            }
            _ => return Err(syn::Error::new_spanned(var_pat, "Expected a simple identifier for the loop variable (e.g., `i`). Patterns are not supported here.")),
        };
        
        let in_kw: kw::in_kw = input.parse()?;
        let iterable: Iterable = input.parse()?;
        let body: Block = input.parse()?;
        
        let else_clause: Option<ElseClause> = if input.peek(kw::else_kw) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(PythonicForInput {
            var,
            in_kw,
            iterable,
            body,
            else_clause,
        })
    }
}


#[proc_macro]
pub fn pythonic_for(input: TokenStream) -> TokenStream {
    let parsed_input = match syn::parse::<PythonicForInput>(input) {
        Ok(pi) => pi,
        Err(e) => return e.to_compile_error().into(),
    };

    let var_ident = parsed_input.var;
    let user_body = parsed_input.body;
    
    // Create a unique label for this specific loop invocation
    let loop_label_str = format!("pythonic_for_loop_{}", var_ident); 
    let loop_label = Lifetime::new(&loop_label_str, var_ident.span());


    let loop_logic = match parsed_input.iterable {
        Iterable::Expr(iterable_expr) => {
            quote! {
                #loop_label: for #var_ident in #iterable_expr {
                    pythonic_for_proc_macros::transform_body!(#loop_label, { #user_body })
                }
            }
        }
        Iterable::RangeWithStep { range_expr, step_clause, .. } => {
            let (start_expr, end_expr, inclusive) = match range_expr {
                Expr::Range(expr_range) => {
                    let start = expr_range.start.as_deref();
                    let end = expr_range.end.as_deref();
                    let limits = expr_range.limits;

                    if start.is_none() {
                        return syn::Error::new_spanned(&expr_range, "Range must have a start value for stepped iteration.")
                                        .to_compile_error().into();
                    }
                    if end.is_none() {
                         return syn::Error::new_spanned(&expr_range, "Range must have an end value for stepped iteration.")
                                        .to_compile_error().into();
                    }
                    (start.unwrap().clone(), end.unwrap().clone(), matches!(limits, RangeLimits::Closed(_)))
                }
                ref other_expr => {
                    return syn::Error::new_spanned(
                        other_expr,
                        "Expected a range expression (e.g., `0..10` or `1..=5`) when 'step' is used."
                    ).to_compile_error().into();
                }
            };
            
            let step_val_expr = step_clause.step_expr;
            quote! {
                let __start = #start_expr;
                let __end = #end_expr;
                let __step = #step_val_expr;
                let mut __current = __start;

                if __step == 0 {
                    // TODO: Consider compile_error! or specific runtime panic for step = 0.
                }

                if __step > 0 {
                    #loop_label: while if #inclusive { __current <= __end } else { __current < __end } {
                        let #var_ident = __current;
                        pythonic_for_proc_macros::transform_body!(#loop_label, { #user_body })
                        __current += __step;
                    }
                } else if __step < 0 {
                    #loop_label: while if #inclusive { __current >= __end } else { __current > __end } {
                        let #var_ident = __current;
                        pythonic_for_proc_macros::transform_body!(#loop_label, { #user_body })
                        __current += __step; 
                    }
                }
                // If step is 0, and range is not empty, it's an infinite loop.
            }
        }
    };

    let else_block_logic = if let Some(else_c) = parsed_input.else_clause {
        let else_body_content = else_c.else_body;
        quote! {
            if !_break_occurred && __result.is_ok() {
                #else_body_content
            }
        }
    } else {
        quote! {}
    };

    let final_code = quote! {
        {
            let mut _break_occurred = false;
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #loop_logic
            }));
            
            #else_block_logic
        }
    };

    final_code.into()
}
