use windows::Win32::{
    Foundation::*, Networking::NetworkListManager::*, System::Com::*, UI::WindowsAndMessaging::*,
};
use windows_core::Interface;

use crate::NetworkStatus;

use super::events::WM_GPUI_NETWORK_CHANGE;

pub(crate) fn query_network_status() -> NetworkStatus {
    unsafe {
        let manager: Result<INetworkListManager, _> =
            CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL);
        match manager {
            Ok(manager) => match manager.GetConnectivity() {
                Ok(connectivity) => {
                    let has_ipv4 = (connectivity.0 & NLM_CONNECTIVITY_IPV4_INTERNET.0) != 0;
                    let has_ipv6 = (connectivity.0 & NLM_CONNECTIVITY_IPV6_INTERNET.0) != 0;
                    if has_ipv4 || has_ipv6 {
                        NetworkStatus::Online
                    } else {
                        NetworkStatus::Offline
                    }
                }
                Err(_) => NetworkStatus::Online,
            },
            Err(_) => NetworkStatus::Online,
        }
    }
}

pub(crate) fn start_network_monitoring(platform_hwnd: HWND, _validation_number: usize) {
    let hwnd_raw = platform_hwnd.0 as usize;
    std::thread::Builder::new()
        .name("NetworkMonitor".to_owned())
        .spawn(move || {
            let platform_hwnd = HWND(hwnd_raw as *mut _);
            unsafe {
                let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            }
            let manager: Result<INetworkListManager, _> =
                unsafe { CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL) };
            let Ok(manager) = manager else {
                log::warn!("Failed to create INetworkListManager for monitoring");
                return;
            };
            let cpc: Result<IConnectionPointContainer, _> = manager.cast();
            let Ok(cpc) = cpc else {
                log::warn!("Failed to get IConnectionPointContainer");
                return;
            };
            let cp = unsafe { cpc.FindConnectionPoint(&INetworkListManagerEvents::IID) };
            let Ok(cp) = cp else {
                log::warn!("Failed to find INetworkListManagerEvents connection point");
                return;
            };
            let sink = NetworkEventSink { platform_hwnd };
            let sink: INetworkListManagerEvents = sink.into();
            let cookie = unsafe { cp.Advise(&sink) };
            let Ok(_cookie) = cookie else {
                log::warn!("Failed to advise network events");
                return;
            };

            let mut msg = MSG::default();
            loop {
                let result = unsafe { GetMessageW(&mut msg, None, 0, 0) };
                if !result.as_bool() {
                    break;
                }
                unsafe {
                    DispatchMessageW(&msg);
                }
            }
        })
        .ok();
}

#[windows::core::implement(INetworkListManagerEvents)]
struct NetworkEventSink {
    platform_hwnd: HWND,
}

impl INetworkListManagerEvents_Impl for NetworkEventSink_Impl {
    fn ConnectivityChanged(&self, _newconnectivity: NLM_CONNECTIVITY) -> windows::core::Result<()> {
        unsafe {
            let _ = PostMessageW(
                Some(self.platform_hwnd),
                WM_GPUI_NETWORK_CHANGE,
                WPARAM(0),
                LPARAM(0),
            );
        }
        Ok(())
    }
}
