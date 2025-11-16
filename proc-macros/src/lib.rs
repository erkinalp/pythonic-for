/*
This code is 100% AI-generated.
It is out of copyright outside the Commonwealth due to the lack of legal personhood of the creator.
*/
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Block, visit_mut::{self, VisitMut}, Token, Ident, Expr, Result,
    parse::{Parse, ParseStream}, RangeLimits, Pat, Lifetime,
    ext::IdentExt,
};

/// Defines the input structure for the `transform_body` macro.
/// Expects an input in the format `label, { body }`.
struct TransformBodyInput {
    /// The lifetime (label) associated with the loop.
    label: Lifetime,
    /// The comma separating the label and the body.
    _comma: Token![,],
    /// The block of code to be transformed.
    body: Block,
}

impl Parse for TransformBodyInput {
    /// Parses a `TransformBodyInput` from a TokenStream.
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            label: input.parse()?,
            _comma: input.parse()?,
            body: input.parse()?,
        })
    }
}

/// A visitor used to transform `break` statements within Pythonic `for` loops.
struct BreakTransformer {
    /// Flag indicating if the current context is within the body of a Pythonic `for` loop.
    in_pythonic_loop_body: bool, 
    /// The lifetime (label) of the current loop.
    current_loop_label: Lifetime,
}

impl BreakTransformer {
    /// Creates a new `BreakTransformer` for a given loop label.
    fn new(loop_label: Lifetime) -> Self {
        Self {
            in_pythonic_loop_body: true, 
            current_loop_label: loop_label,
        }
    }
}

impl VisitMut for BreakTransformer {
    /// Visits an `ExprForLoop` node, setting the `in_pythonic_loop_body` flag for its body.
    fn visit_expr_for_loop_mut(&mut self, node: &mut syn::ExprForLoop) {
        let originally_in_pythonic_loop_body = self.in_pythonic_loop_body;
        self.in_pythonic_loop_body = false; 
        visit_mut::visit_expr_for_loop_mut(self, node);
        self.in_pythonic_loop_body = originally_in_pythonic_loop_body; 
    }

    /// Visits an `ExprWhile` node, setting the `in_pythonic_loop_body` flag for its body.
    fn visit_expr_while_mut(&mut self, node: &mut syn::ExprWhile) {
        let originally_in_pythonic_loop_body = self.in_pythonic_loop_body;
        self.in_pythonic_loop_body = false;
        visit_mut::visit_expr_while_mut(self, node);
        self.in_pythonic_loop_body = originally_in_pythonic_loop_body;
    }

    /// Visits an `ExprLoop` node, setting the `in_pythonic_loop_body` flag for its body.
    fn visit_expr_loop_mut(&mut self, node: &mut syn::ExprLoop) {
        let originally_in_pythonic_loop_body = self.in_pythonic_loop_body;
        self.in_pythonic_loop_body = false;
        visit_mut::visit_expr_loop_mut(self, node);
        self.in_pythonic_loop_body = originally_in_pythonic_loop_body;
    }
    
    /// Visits an `Expr` node, transforming `break` statements if found within a Pythonic loop body.
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Break(expr_break) = node {
            if self.in_pythonic_loop_body {
                let user_label = &expr_break.label;
                let break_expr_val = &expr_break.expr;

                // Determine the target label for the break statement.
                let target_label_to_quote = match user_label {
                    Some(l) => quote!(#l),
                    None => {
                        let l = &self.current_loop_label;
                        quote!(#l)
                    }
                };
            
                // Construct the new break expression with the `_break_occurred` flag.
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
                
                // Parse the new expression and replace the original node.
                match syn::parse2::<Expr>(new_expr_tokens) {
                    Ok(new_expr_block) => {
                        *node = new_expr_block; 
                        return; 
                    }
                    Err(e) => {
                        // If parsing fails, generate a compile-time error message.
                        let error_ts = syn::Error::new_spanned(
                            &*node, // Borrow instead of move to allow subsequent assignment
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


/// Procedure macro to transform a block of code by replacing `break` statements
/// with versions that set a `_break_occurred` flag.
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

/// Custom keywords used in the `pythonic_for` macro.
mod kw {
    syn::custom_keyword!(step);
}

/// Represents the `step` clause in a Pythonic `for` loop.
struct StepClause {
    /// The `step` keyword.
    #[allow(dead_code)]
    step_kw: kw::step,
    /// The `=` token.
    #[allow(dead_code)]
    eq_token: Token![=],
    /// The expression for the step value.
    step_expr: Expr,
}

impl Parse for StepClause {
    /// Parses a `StepClause` from a TokenStream.
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            step_kw: input.parse()?,
            eq_token: input.parse()?,
            step_expr: input.parse()?,
        })
    }
}

/// Represents the iterable part of a Pythonic `for` loop.
enum Iterable {
    /// A simple expression (e.g., `my_list`).
    Expr(Expr), 
    /// A range expression with an optional step clause (e.g., `0..10, step=2`).
    RangeWithStep {
        /// The range expression.
        range_expr: Expr, 
        /// The comma separating the range and the step clause.
        #[allow(dead_code)]
        comma_token: Token![,],
        /// The step clause.
        step_clause: StepClause,
    },
}

impl Parse for Iterable {
    /// Parses an `Iterable` from a TokenStream.
    fn parse(input: ParseStream) -> Result<Self> {
        let expr: Expr = Expr::parse_without_eager_brace(input)?;
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

/// Represents either `else` or `final` keyword.
#[allow(dead_code)]
enum ElseOrFinalKw {
    Else(Token![else]),
    Final(Ident),
}

/// Represents the `else` or `final` clause of a Pythonic `for` loop.
struct ElseClause {
    /// The `else` or `final` keyword.
    #[allow(dead_code)]
    keyword: ElseOrFinalKw,
    /// The body of the `else`/`final` block.
    else_body: Block,
}

impl Parse for ElseClause {
    /// Parses an `ElseClause` from a TokenStream.
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword = if input.peek(Token![else]) {
            ElseOrFinalKw::Else(input.parse()?)
        } else {
            let fork = input.fork();
            if let Ok(ident) = fork.call(Ident::parse_any) {
                if ident == "final" {
                    let _ = input.call(Ident::parse_any)?;
                    ElseOrFinalKw::Final(ident)
                } else {
                    return Err(syn::Error::new_spanned(ident, "expected `else` or `final`"));
                }
            } else {
                return Err(input.error("expected `else` or `final`"));
            }
        };
        
        Ok(ElseClause {
            keyword,
            else_body: input.parse()?,
        })
    }
}

/// Defines the input structure for the `pythonic_for` macro.
/// Expects an input in the format `var in iterable { body } [else { else_body }]`.
struct PythonicForInput {
    /// The loop variable identifier.
    var: Ident,
    /// The `in` keyword.
    #[allow(dead_code)]
    in_token: Token![in],
    /// The iterable expression.
    iterable: Iterable,
    /// The body of the loop.
    body: Block,
    /// An optional `else` clause.
    else_clause: Option<ElseClause>,
}

impl Parse for PythonicForInput {
    /// Parses a `PythonicForInput` from a TokenStream.
    fn parse(input: ParseStream) -> Result<Self> {
        let var_pat: Pat = Pat::parse_single(input)?;
        let var = match var_pat {
            Pat::Ident(pat_ident) => {
                // Validate that the loop variable is a simple identifier.
                if !pat_ident.attrs.is_empty() || pat_ident.by_ref.is_some() || pat_ident.mutability.is_some() || pat_ident.subpat.is_some() {
                     return Err(syn::Error::new_spanned(pat_ident, "Loop variable must be a simple identifier (e.g., `i`), not a complex pattern."));
                }
                pat_ident.ident
            }
            _ => return Err(syn::Error::new_spanned(var_pat, "Expected a simple identifier for the loop variable (e.g., `i`). Patterns are not supported here.")),
        };
        
        let in_token: Token![in] = input.parse()?;
        let iterable: Iterable = input.parse()?;
        let body: Block = input.parse()?;
        
        // Check for an optional `else` or `final` clause.
        let else_clause: Option<ElseClause> = if input.peek(Token![else]) {
            Some(input.parse()?)
        } else {
            let fork = input.fork();
            if let Ok(ident) = fork.call(Ident::parse_any) {
                if ident == "final" {
                    Some(input.parse()?)
                } else {
                    None
                }
            } else {
                None
            }
        };

        // Ensure all tokens have been consumed
        if !input.is_empty() {
            return Err(input.error("unexpected tokens after pythonic_for invocation"));
        }

        Ok(PythonicForInput {
            var,
            in_token,
            iterable,
            body,
            else_clause,
        })
    }
}


/// Procedure macro to implement Pythonic-style `for` loops in Rust.
/// Supports simple iterables, ranges with optional steps, and optional `else` clauses.
#[proc_macro]
pub fn pythonic_for(input: TokenStream) -> TokenStream {
    let parsed_input = match syn::parse::<PythonicForInput>(input) {
        Ok(pi) => pi,
        Err(e) => return e.to_compile_error().into(),
    };

    let var_ident = parsed_input.var;
    let user_body = parsed_input.body;
    
    // Create a unique label for this specific loop invocation.
    let loop_label_str = format!("'pythonic_for_loop_{}", var_ident); 
    let loop_label = Lifetime::new(&loop_label_str, var_ident.span());


    // Generate the loop logic based on the type of iterable.
    let loop_logic = match parsed_input.iterable {
        Iterable::Expr(iterable_expr) => {
            // Standard `for` loop over an expression.
            quote! {
                #loop_label: for #var_ident in #iterable_expr {
                    pythonic_for_proc_macros::transform_body!(#loop_label, { #user_body })
                }
            }
        }
        Iterable::RangeWithStep { range_expr, step_clause, .. } => {
            // `for` loop over a range with an optional step.
            let (start_expr, end_expr, inclusive) = match range_expr {
                Expr::Range(expr_range) => {
                    let start = expr_range.start.as_deref();
                    let end = expr_range.end.as_deref();
                    let limits = expr_range.limits;

                    // Ensure start and end values are present for stepped iteration.
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
                    // Error if the iterable is not a range expression.
                    return syn::Error::new_spanned(
                        other_expr,
                        "Expected a range expression (e.g., `0..10` or `1..=5`) when 'step' is used."
                    ).to_compile_error().into();
                }
            };
            
            let step_val_expr = step_clause.step_expr;
            // Generate a `while` loop based on the step direction.
            quote! {
                let __start = #start_expr;
                let __end = #end_expr;
                let __step = #step_val_expr;
                let mut __current = __start;

                if __step == 0 {
                    panic!("pythonic_for: step argument must not be zero (matches Python's ValueError for range(step=0))");
                }

                if __step > 0 {
                    // Positive step: iterate upwards.
                    #loop_label: while if #inclusive { __current <= __end } else { __current < __end } {
                        let #var_ident = __current;
                        pythonic_for_proc_macros::transform_body!(#loop_label, { #user_body });
                        __current += __step;
                    }
                } else if __step < 0 {
                    // Negative step: iterate downwards.
                    #loop_label: while if #inclusive { __current >= __end } else { __current > __end } {
                        let #var_ident = __current;
                        pythonic_for_proc_macros::transform_body!(#loop_label, { #user_body });
                        __current += __step; 
                    }
                }
                // If step is 0, and range is not empty, it's an infinite loop.
            }
        }
    };

    // Generate the `else` block logic if present.
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

    // Combine the loop logic, else block, and panic handling.
    let final_code = quote! {
        {
            let mut _break_occurred = false;
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #loop_logic
            }));
            
            #else_block_logic
            
            if let Err(panic_payload) = __result {
                std::panic::resume_unwind(panic_payload);
            }
        }
    };

    final_code.into()
}
