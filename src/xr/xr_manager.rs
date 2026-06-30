pub struct XRManager {
    instance: openxr::Instance,
    system: Option<openxr::SystemId>,
    session: Option<openxr::Session<openxr::Vulkan>>,
    frame_waiter: Option<openxr::FrameWaiter>,
    frame_stream: Option<openxr::FrameStream<openxr::Vulkan>>,
    views: Vec<openxr::View>,
}
impl XRManager {
    pub fn new() -> Result<Self, openxr::sys::Result> {
        let entry = unsafe { openxr::Entry::load().expect("openxr not found") };

        let extensions = openxr::ExtensionSet::default();
        let instance = entry.create_instance(
            &openxr::ApplicationInfo {
                application_name: "Caevern",
                application_version: 1,
                engine_name: "Caevern",
                engine_version: 1,
                api_version: openxr::CURRENT_API_VERSION,
            },
            &extensions,
            &[],
        )?;

        Ok(Self {
            instance,
            system: None,
            session: None,
            frame_waiter: None,
            frame_stream: None,
            views: Vec::new(),
        })
    }
}