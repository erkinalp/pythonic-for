use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Block, ExprBreak, visit_mut::{self, VisitMut}};

struct BreakTransformer {
    loop_depth: usize,
}

impl BreakTransformer {
    fn new() -> Self {
        Self { loop_depth: 0 }
    }
}

impl VisitMut for BreakTransformer {
    fn visit_expr_for_loop_mut(&mut self, node: &mut syn::ExprForLoop) {
        self.loop_depth += 1;
        visit_mut::visit_expr_for_loop_mut(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_while_mut(&mut self, node: &mut syn::ExprWhile) {
        self.loop_depth += 1;
        visit_mut::visit_expr_while_mut(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_loop_mut(&mut self, node: &mut syn::ExprLoop) {
        self.loop_depth += 1;
        visit_mut::visit_expr_loop_mut(self, node);
        self.loop_depth -= 1;
    }

    fn visit_expr_break_mut(&mut self, node: &mut ExprBreak) {
        if self.loop_depth == 1 {
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
