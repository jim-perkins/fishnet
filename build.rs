fn main() {
    auditable_build::collect_dependency_list();

    #[cfg(windows)]
    winres::WindowsResource::new().compile().unwrap();
}
