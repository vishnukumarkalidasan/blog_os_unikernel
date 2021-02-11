use bootloader_locator::locate_bootloader;

fn main() {
    let bootloader_manifest = locate_bootloader("bootloader").unwrap();
    dbg!(bootloader_manifest);
}
