use crate::{BiometricKind, BiometricStatus};
use cocoa::base::{id, nil, BOOL, YES};
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl};

const LA_POLICY_BIOMETRICS: i64 = 1;

#[link(name = "LocalAuthentication", kind = "framework")]
unsafe extern "C" {}

pub fn biometric_status() -> BiometricStatus {
    unsafe {
        let context: id = msg_send![class!(LAContext), new];
        let mut error: id = nil;
        let can_evaluate: BOOL = msg_send![
            context,
            canEvaluatePolicy: LA_POLICY_BIOMETRICS
            error: &mut error
        ];
        let _: () = msg_send![context, release];
        if can_evaluate == YES {
            BiometricStatus::Available(BiometricKind::TouchId)
        } else {
            BiometricStatus::Unavailable
        }
    }
}

pub fn authenticate_biometric(reason: &str, callback: Box<dyn FnOnce(bool) + Send>) {
    unsafe {
        let context: id = msg_send![class!(LAContext), new];
        let reason_ns = NSString::alloc(nil).init_str(reason);
        let _: id = msg_send![reason_ns, autorelease];

        let callback = std::sync::Mutex::new(Some(callback));

        let block = block::ConcreteBlock::new(move |success: BOOL, _error: id| {
            if let Some(cb) = callback.lock().ok().and_then(|mut guard| guard.take()) {
                cb(success == YES);
            }
        });
        let block = block.copy();

        let _: () = msg_send![
            context,
            evaluatePolicy: LA_POLICY_BIOMETRICS
            localizedReason: reason_ns
            reply: &*block
        ];

        let _: () = msg_send![context, release];
    }
}
