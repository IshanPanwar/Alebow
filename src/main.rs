use crate::app::AppData;
mod app;
use smithay_client_toolkit::{
    compositor::CompositorHandler,
    reexports::{
        calloop::EventLoop,
        calloop_wayland_source::WaylandSource
    }
};
use std::time::Duration;
use wayland_client::{
    globals::registry_queue_init,
    protocol::wl_buffer,
    Connection, QueueHandle,
};

fn main() {
    //env_logger::init();

    let conn = Connection::connect_to_env().unwrap();

    let (globals, event_queue) = registry_queue_init(&conn).unwrap();
    let qh: QueueHandle<AppData> = event_queue.handle();
    let mut event_loop: EventLoop<AppData> =
    EventLoop::try_new().expect("Failed to initialize the event loop!");

    let (mut app_data, qh) = AppData::init(event_loop.handle(), conn.clone(), qh, globals);

    app_data.session_lock =
    Some(app_data.session_lock_state.lock(&qh).expect("ext-session-lock not supported"));

    WaylandSource::new(conn.clone(), event_queue).insert(event_loop.handle()).unwrap();

    loop {
        event_loop.dispatch(Duration::from_millis(16), &mut app_data).unwrap();
        app_data.frame(&conn, &qh, &app_data.lock_surfaces[0].clone().wl_surface(), 16u32);

        if app_data.exit {
            //println!("The end");
            break;
        }
    }
}

smithay_client_toolkit::delegate_compositor!(AppData);
smithay_client_toolkit::delegate_output!(AppData);
smithay_client_toolkit::delegate_session_lock!(AppData);
smithay_client_toolkit::delegate_shm!(AppData);
smithay_client_toolkit::delegate_registry!(AppData);
wayland_client::delegate_noop!(AppData: ignore wl_buffer::WlBuffer);
