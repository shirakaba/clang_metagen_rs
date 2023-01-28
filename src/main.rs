extern crate clang;

use clang::*;

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a header file into a translation unit
    // let tu = index.parser("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/Foundation.framework/Headers/Foundation.h").parse().unwrap();
    // let tu = index.parser("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/Foundation.framework/Modules/module.modulemap").arguments(&["-fmodules"]).parse().unwrap();
    let tu = index.parser("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/Foundation.framework/Headers/Foundation.h").parse().unwrap();
    
    println!("diagnostics {:?}", tu.get_diagnostics());

    // Get the structs in this translation unit
    let structs = tu.get_entity().get_children().into_iter().filter(|e| {
        e.get_kind() == EntityKind::ObjCInterfaceDecl
    }).collect::<Vec<_>>();

    // Print information about the structs
    for struct_ in structs {
        let type_ =  struct_.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();
        println!("struct: {:?} (size: {} bytes)", struct_.get_name().unwrap(), size);

        for field in struct_.get_children() {
            let name = field.get_name().unwrap();
            let offset = type_.get_offsetof(&name).unwrap();
            println!("    field: {:?} (offset: {} bits)", name, offset);
        }
    }
}
