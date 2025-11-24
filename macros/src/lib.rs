use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Lit, Meta, MetaNameValue, parse_macro_input};

/// Attribute macro to mark a function as a syscall handler
///
/// # Example
/// ```ignore
/// #[moss(syscall = 5)]
/// fn my_syscall_handler(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64) -> usize {
///     // Implementation
///     Ok(0)
/// }
/// ```
#[proc_macro_attribute]
pub fn moss(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Parse the attribute to get the syscall number
    let attr_meta = parse_macro_input!(attr as Meta);

    let syscall_number = match attr_meta {
        Meta::NameValue(MetaNameValue { path, value, .. }) => {
            // Check that the path is "syscall"
            if !path.is_ident("syscall") {
                panic!(
                    "Expected #[moss(syscall = N)], found #[moss({} = ...)]",
                    path.get_ident()
                        .map(|i| i.to_string())
                        .unwrap_or_else(|| "?".to_string())
                );
            }

            // Extract the number from the value
            match value {
                syn::Expr::Lit(expr_lit) => match expr_lit.lit {
                    Lit::Int(lit_int) => lit_int
                        .base10_parse::<u32>()
                        .expect("Invalid syscall number"),
                    _ => panic!("Expected a number literal, e.g., #[moss(syscall = 5)]"),
                },
                _ => panic!("Expected a number literal, e.g., #[moss(syscall = 5)]"),
            }
        }
        _ => panic!("Expected format: #[moss(syscall = N)], e.g., #[moss(syscall = 5)]"),
    };

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;
    let fn_asyncness = &input_fn.sig.asyncness;

    // Check if the function is async - it MUST be
    if fn_asyncness.is_none() {
        panic!(
            "The #[moss] attribute can only be used on async functions. Please add 'async' keyword to your function."
        );
    }

    // Create a wrapper function that returns a boxed future
    let wrapper_name = quote::format_ident!("__moss_wrapper_{}", fn_name);
    let handler_name =
        quote::format_ident!("__SYSCALL_HANDLER_{}", fn_name.to_string().to_uppercase());
    let hex = format!("{:#x}", syscall_number);
    let log = format!("Syscall: {hex}");

    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;

    let expanded = quote! {
        // Original async function
        #(#fn_attrs)*
        #[allow(dead_code)]
        #fn_vis async fn #fn_name(#fn_inputs) #fn_output {
            log::trace!(#log);
            #fn_block
        }

        // Wrapper function that returns a boxed future
        #[doc(hidden)]
        fn #wrapper_name(
            arg1: u64,
            arg2: u64,
            arg3: u64,
            arg4: u64,
            arg5: u64,
            arg6: u64
        ) -> ::core::pin::Pin<::alloc::boxed::Box<dyn ::futures::Future<Output = Result<usize>> + Send>> {
            ::alloc::boxed::Box::pin(#fn_name(arg1, arg2, arg3, arg4, arg5, arg6))
        }

        // Submit this handler to the inventory
        #[doc(hidden)]
        #[linkme::distributed_slice(crate::kernel::syscall::SYSCALLS)]
        static #handler_name: crate::kernel::syscall::SyscallHandler = crate::kernel::syscall::SyscallHandler {
            number: #syscall_number,
            hex: #hex,
            name: stringify!(#fn_name),
            handler: #wrapper_name,
        };
    };

    TokenStream::from(expanded)
}
