use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, ExprBreak, visit_mut::{self, VisitMut}};

struct BreakTransformer {
    loop_depth: usize,
    in_pythonic_for: bool,
}

impl BreakTransformer {
    fn new() -> Self {
        Self { 
            loop_depth: 0,
            in_pythonic_for: true,
        }
    }
}

impl VisitMut for BreakTransformer {
    fn visit_expr_for_loop_mut(&mut self, node: &mut syn::ExprForLoop) {
        let was_in_pythonic_for = self.in_pythonic_for;
        
        if let Some(label) = &node.label {
            let label_str = format!("{}", quote!(#label));
            if label_str.contains("pythonic_for_loop") {
                self.in_pythonic_for = true;
            }
        }
        
        self.loop_depth += 1;
        visit_mut::visit_expr_for_loop_mut(self, node);
        self.loop_depth -= 1;
        
        self.in_pythonic_for = was_in_pythonic_for;
    }

    fn visit_expr_while_mut(&mut self, node: &mut syn::ExprWhile) {
        let was_in_pythonic_for = self.in_pythonic_for;
        
        if let Some(label) = &node.label {
            let label_str = format!("{}", quote!(#label));
            if label_str.contains("pythonic_while_loop") {
                self.in_pythonic_for = true;
            }
        }
        
        self.loop_depth += 1;
        visit_mut::visit_expr_while_mut(self, node);
        self.loop_depth -= 1;
        
        self.in_pythonic_for = was_in_pythonic_for;
    }

    fn visit_expr_loop_mut(&mut self, node: &mut syn::ExprLoop) {
        let was_in_pythonic_for = self.in_pythonic_for;
        
        if let Some(label) = &node.label {
            let label_str = format!("{}", quote!(#label));
            if label_str.contains("pythonic_while_loop") || label_str.contains("pythonic_for_loop") {
                self.in_pythonic_for = true;
            }
        }
        
        self.loop_depth += 1;
        visit_mut::visit_expr_loop_mut(self, node);
        self.loop_depth -= 1;
        
        self.in_pythonic_for = was_in_pythonic_for;
    }

    fn visit_expr_break_mut(&mut self, node: &mut ExprBreak) {
        if self.in_pythonic_for {
            let label = &node.label;
            let expr = &node.expr;
            
            let new_expr = if let Some(label) = label {
                if let Some(expr) = expr {
                    quote! {{
                        _break_occurred = true;
                        break #label #expr;
                    }}
                } else {
                    quote! {{
                        _break_occurred = true;
                        break #label;
                    }}
                }
            } else if let Some(expr) = expr {
                quote! {{
                    _break_occurred = true;
                    break #expr;
                }}
            } else {
                quote! {{
                    _break_occurred = true;
                    break;
                }}
            };
            
            match syn::parse2(new_expr) {
                Ok(new_node) => *node = new_node,
                Err(e) => {
                    eprintln!("Failed to transform break statement: {}", e);
                }
            }
        }
        
        visit_mut::visit_expr_break_mut(self, node);
    }
}

#[proc_macro]
pub fn transform_body(input: TokenStream) -> TokenStream {
    let mut block = parse_macro_input!(input as Block);
    
    let mut transformer = BreakTransformer::new();
    transformer.visit_block_mut(&mut block);
    
    let expanded = quote! {
        #block
    };
    
    expanded.into()
}
