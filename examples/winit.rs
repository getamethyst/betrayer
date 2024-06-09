use anyhow::Result;
use betrayer::winit::WinitTrayIconBuilderExt;
use betrayer::{Icon, Menu, MenuItem, TrayEvent, TrayIconBuilder};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Signal {
    Profile(u32),
    Null,
    Quit,
    CheckIcon
}

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_module_level("betrayer", LevelFilter::Trace)
        .with_level(LevelFilter::Debug)
        .init()?;

    let event_loop = EventLoopBuilder::with_user_event().build()?;

    let mut selected = 0;
    let mut icon_checked = false;

    let tray = TrayIconBuilder::new()
        .with_icon(Icon::from_rgba(vec![255; 32 * 32 * 4], 32, 32)?)
        .with_tooltip("Demo System Tray")
        .with_menu(build_menu(selected, icon_checked))
        // with `winit` feature:
        .build_event_loop(&event_loop, |e| Some(e))?;
        // without:
        //.build({
        //    let proxy = event_loop.create_proxy();
        //    move |s| {let _ = proxy.send_event(s); }
        //})?;

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run(|event, evtl| match event {
        Event::UserEvent(event) => {
            log::info!("tray event: {:?}", event);
            if let TrayEvent::Menu(signal) = event {
                match signal {
                    Signal::Profile(i) => {
                        if selected != i {
                            selected = i;
                            tray.set_tooltip(format!("Active Profile: {selected}"));
                            tray.set_menu(build_menu(selected, icon_checked));
                        }
                    }
                    Signal::Null => {}
                    Signal::Quit => evtl.exit(),
                    Signal::CheckIcon => {
                        icon_checked = !icon_checked;
                        tray.set_menu(build_menu(selected, icon_checked));
                    }
                }
            }
        }
        _ => {}
    })?;
    Ok(())
}

fn build_menu(selected: u32, icon_checked: bool) -> Menu<Signal> {
    Menu::new([
        MenuItem::menu(
            "Profiles",
            (0..5).map(|i| {
                let name = format!("Profile {}", i + 1);
                let signal = Signal::Profile(i);
                let checked = selected == i;

                return MenuItem::check_button(name, signal, checked, false, (None, None));
            })
        ),
        MenuItem::separator(),
        MenuItem::check_button("Icon", Signal::CheckIcon, icon_checked, false, (Some(Icon::from_rgba(vec![0; 16 * 16 * 4], 16, 16).unwrap()), Some(Icon::from_rgba(vec![255; 16 * 16 * 4], 16, 16).unwrap()))),
        MenuItem::button("Disabled", Signal::Null, true, None),
        MenuItem::button("Quit", Signal::Quit, false, None)
    ])
}
