

use std::{env, fs::{self, File}, path::Path, io::Write};

use bindgen::{Builder, CodegenConfig};

const PROJECT_DIRECTORY_NAME: &str = "binding-generator";

const BINDING_DIR: &str = "../xlib-sys/src/generated_bindings";
const C_LIBRARIES_DIR: &str = "../modules";

fn bindgen_global_options() -> Builder {
    let mut b = Builder::default()
        .layout_tests(false)
        .derive_copy(true)
        .rust_target(bindgen::LATEST_STABLE_RUST);

    for d in fs::read_dir(C_LIBRARIES_DIR).unwrap() {
        let mut library = d.unwrap().path();
        library.push("include");

        if !library.exists() {
            continue;
        }

        if !library.is_dir() {
            panic!("{:?} is not a directory.", library);
        }

        let absolute_path = fs::canonicalize(library).unwrap().into_os_string().into_string().unwrap();
        let clang_arg = format!("--include-directory={}", absolute_path);
        b = b.clang_arg(clang_arg);
    }

    b
}

fn disable_function_generation() -> CodegenConfig {
    let mut settings = CodegenConfig::all();
    settings.set(CodegenConfig::FUNCTIONS, false);
    settings
}

fn generate_only_functions() -> CodegenConfig {
    let mut settings = CodegenConfig::empty();
    settings.set(CodegenConfig::FUNCTIONS, true);
    settings
}


fn write_bindings(output_file_name: &str, output_start_contents: &str, builder: Builder) {
    let output = Path::new(BINDING_DIR).join(output_file_name);
    let mut file = File::create(output).unwrap();
    file.write_all(output_start_contents.as_bytes()).unwrap();

    builder.generate().unwrap().write(Box::new(file)).unwrap();
}

fn header_path(library_dir_name: &str, include_path: &str) -> String {
    let mut path = Path::new(C_LIBRARIES_DIR).to_path_buf();
    path.push(library_dir_name);
    path.push("include");
    path.push(include_path);
    fs::canonicalize(path).unwrap().into_os_string().into_string().unwrap()
}

fn main() {
    env_logger::init();

    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.file_name().unwrap();
    if  current_dir != PROJECT_DIRECTORY_NAME {
        panic!("Run binding generator from it's project directory '{}'.", PROJECT_DIRECTORY_NAME);
    }

    // libx11

    // TODO: Xlib-xcb.h

    let function_regex = "^X[A-Za-z]*$";
    let var_and_type_regex = "^([A-Z]|_Xrm)[A-Za-z]*$";

    write_bindings(
        "xlib.rs",
        "",
        bindgen_global_options()
            .header(header_path("libx11", "X11/Xlib.h"))
            .header(header_path("libx11", "X11/cursorfont.h"))
            .header(header_path("libx11", "X11/ImUtil.h"))
            .header(header_path("libx11", "X11/Xcms.h"))
            .header(header_path("libx11", "X11/XKBlib.h"))
            .header(header_path("libx11", "X11/Xregion.h"))
            .header(header_path("libx11", "X11/Xresource.h"))
            .header(header_path("libx11", "X11/Xutil.h"))
            .with_codegen_config(disable_function_generation())
            .whitelist_var(var_and_type_regex)
            .whitelist_type(var_and_type_regex)
    );

    write_bindings(
        "xlib_functions.rs",
        "use crate::generated_bindings::xlib::*;\n",
        bindgen_global_options()
            .header(header_path("libx11", "X11/Xlib.h"))
            .header(header_path("libx11", "X11/cursorfont.h"))
            .header(header_path("libx11", "X11/ImUtil.h"))
            .header(header_path("libx11", "X11/Xcms.h"))
            .header(header_path("libx11", "X11/XKBlib.h"))
            .header(header_path("libx11", "X11/Xregion.h"))
            .header(header_path("libx11", "X11/Xresource.h"))
            .header(header_path("libx11", "X11/Xutil.h"))
            .with_codegen_config(generate_only_functions())
            .whitelist_function(function_regex)
    );

    // libxfixes

    let function_regex = "^XFixes[A-Za-z]*$";
    let var_and_type_regex = "^XFixes[A-Za-z]*$|^XserverRegion$|^PointerBarrier$";

    write_bindings(
        "xfixes.rs",
        "use crate::generated_bindings::xlib::*;\n",
        bindgen_global_options()
            .header(header_path("libxfixes", "X11/extensions/Xfixes.h"))
            .with_codegen_config(disable_function_generation())
            .whitelist_var(var_and_type_regex)
            .whitelist_type(var_and_type_regex)
            .whitelist_recursively(false)
    );

    write_bindings(
        "xfixes_functions.rs",
        "use crate::generated_bindings::{xlib::*, xfixes::*};\n",
        bindgen_global_options()
            .header(header_path("libxfixes", "X11/extensions/Xfixes.h"))
            .with_codegen_config(generate_only_functions())
            .whitelist_function(function_regex)
            .whitelist_recursively(false)
    );

    // TODO: xinput

}
