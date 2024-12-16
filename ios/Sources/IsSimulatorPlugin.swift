import SwiftRs
import Tauri
import UIKit
import WebKit

class IsSimulatorPlugin: Plugin {
  @objc public func isSimulator(_ invoke: Invoke) throws {
    #if targetEnvironment(simulator)
      invoke.resolve(["value": true])
    #else
      invoke.resolve(["value": false])
    #endif
  }
}

@_cdecl("init_plugin_is_simulator")
func initPlugin() -> Plugin {
  return IsSimulatorPlugin()
}
