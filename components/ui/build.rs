

fn main() {
     slint_build::compile_with_config(
         "src/hello.slint",
         slint_build::CompilerConfiguration::new()
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
     )
     .unwrap();
    // slint_build::compile("src/hello.slint").unwrap();

    // let config = slint_build::CompilerConfiguration::new()
    //     .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    // slint_build::compile_with_config("ui/printerdemo.slint", config).unwrap();
}
