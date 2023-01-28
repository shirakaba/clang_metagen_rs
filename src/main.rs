extern crate clang;

use clang::*;

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, true);

    // Parse a header file into a translation unit
    let tu = index.parser(
        "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk/System/Library/Frameworks/Foundation.framework/Headers/Foundation.h"
    )
    // Some arguments based on this CLI output I got from running the metadata
    // generator:
    // https://gist.github.com/shirakaba/5fb986174ba856f71249d97a54be2de8
    // The code behind the args-parsing is a bit harder to follow:
    // https://github.com/NativeScript/ios/blob/main/metadata-generator/build-step-metadata-generator.py
    // https://github.com/NativeScript/ios/blob/main/metadata-generator/src/main.cpp
    .arguments(&[
        "-Xclang",
        
        // You can determine this path using: xcrun --sdk iphoneos --show-sdk-path
        "-isysroot", "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk",

        // Doesn't seem to be necessary. Found in an unrelated project.
        // "-arch", "arm64",

        "-x", "objective-c",
        
        "-fno-objc-arc",
        "-fmodule-maps",
        "-ferror-limit=0",
        
        "-Wno-unknown-pragmas",
        "-Wno-ignored-attributes",
        "-Wno-nullability-completeness",
        "-Wno-expansion-to-defined",

        "-std=gnu99",
        // This is the iPhone simulator I have installed. Your version may
        // differ.
        "-target", "arm64-apple-ios16.2",

        // The below headers I guessed myself. I guess the metadata generator
        // links and includes via calling the clang APIs rather than passing
        // these flags.
        //
        // Include the Foundation umbrella header.
        "-I/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk/System/Library/Frameworks/Foundation.framework/Headers",
        "-I/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk/usr/include",
        // Pass the Frameworks directory.
        "-F/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk/System/Library/Frameworks"
    ])
    .parse()
    .unwrap();

    // Sadly this still complains about: 'sys/types.h' file not found
    println!("diagnostics {:?}", tu.get_diagnostics());

    // Get the structs in this translation unit
    let structs = tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::ObjCInterfaceDecl)
        .collect::<Vec<_>>();

    // Print information about the structs
    for struct_ in structs {
        let type_ = struct_.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();
        println!(
            "struct: {:?} (size: {} bytes)",
            struct_.get_name().unwrap(),
            size
        );

        // for field in struct_.get_children() {
        //     let name = field.get_name().unwrap();
        //     let offset = type_.get_offsetof(&name).unwrap();
        //     println!("    field: {:?} (offset: {} bits)", name, offset);
        // }
    }
}
