use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Error, Expr, ItemFn, ReturnType, Stmt, StmtMacro, spanned::Spanned, token::Semi};

#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).unwrap();

    let signature_output = signature_output_as_result(&ast);
    let statements_output: Result<Vec<Stmt>, Error> = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            Stmt::Expr(e, t) => handle_expression(e, t),
            _ => Ok(s),
        })
        .collect();
    match (statements_output, signature_output) {
        (Ok(stmt), Ok(sig)) => {
            ast.block.stmts = stmt;
            ast.sig.output = sig;
        }
        (Ok(_), Err(err)) => return err.to_compile_error().into(),
        (Err(err), Ok(_)) => return err.to_compile_error().into(),
        (Err(mut stmt_error), Err(sig_error)) => {
            stmt_error.combine(sig_error);
            return stmt_error.to_compile_error().into();
        }
    }

    let last_stmt = ast.block.stmts.pop().unwrap();
    ast.block.stmts.push(last_stmt_as_result(last_stmt));

    ast.to_token_stream().into()
}

fn handle_expression(expression: Expr, semi: Option<Semi>) -> Result<Stmt, Error> {
    match expression {
        Expr::If(mut ex_if) => {
            let new_statements: Result<Vec<Stmt>, Error> = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) => {
                        let output = extract_panic_content(expr_macro);
                        if output.as_ref().map(|v| v.is_empty()).unwrap_or(false) {
                            Err(Error::new(
                                expr_macro.span(),
                                format!(
                                    "please make sure every panic in your function has a message, check: {}",
                                    quote!(#expr_macro)
                                ),
                            ))
                        } else {
                            Ok(output
                                .map(|t| quote! { return Err(#t.to_string()); })
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s))
                        }
                    }
                    _ => Ok(s),
                })
                .collect();
            ex_if.then_branch.stmts = new_statements?;
            Ok(Stmt::Expr(Expr::If(ex_if), semi))
        }
        _ => Ok(Stmt::Expr(expression, semi)),
    }
}

fn extract_panic_content(expr_macro: &StmtMacro) -> Option<proc_macro2::TokenStream> {
    let does_panic = expr_macro
        .mac
        .path
        .segments
        .iter()
        .any(|v| v.ident.to_string().eq("panic"));

    if does_panic {
        Some(expr_macro.mac.tokens.clone())
    } else {
        None
    }
}

fn last_stmt_as_result(last: Stmt) -> Stmt {
    let last_modified = quote! {
        Ok(#last)
    };
    Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

fn signature_output_as_result(ast: &ItemFn) -> Result<ReturnType, Error> {
    let output = match ast.sig.output {
        ReturnType::Default => quote! {
            -> Result<(), String>
        },
        ReturnType::Type(_, ref ty) => {
            if ty.to_token_stream().to_string().contains("Result") {
                return Err(Error::new(
                    ast.sig.span(),
                    format!(
                        "this macro can only be applied to a function that does not return a Result.\nSignature: {}",
                        quote!(#ty)
                    ),
                ));
            }
            quote! {
                -> Result<#ty, String>
            }
        }
    };

    Ok(syn::parse2(output).unwrap())
}
