; Include required NSIS plugins
!include nsDialogs.nsh
!include LogicLib.nsh
!include FileFunc.nsh
!include x64.nsh
!include WordFunc.nsh

; Define variables for custom pages
Var ModelSelectDialog
Var ModelSelectText
Var ModelSelectState
Var LicenseDialog
Var LicenseText
Var LicenseState
Var TelemetryDialog
Var TelemetryText
Var TelemetryState
Var VulkanDialog
Var VulkanText
Var VulkanState

; Custom page for model selection
Page custom ModelSelectPage ModelSelectLeave
Function ModelSelectPage
    nsDialogs::Create 1018
    Pop $ModelSelectDialog
    
    ${If} $ModelSelectDialog == error
        Abort
    ${EndIf}
    
    ; Create model selection dropdown
    ${NSD_CreateDropDown} 0 0 100% 12u
    Pop $ModelSelectText
    
    ; Add model options
    ${NSD_AddString} $ModelSelectText "small-model"
    ${NSD_AddString} $ModelSelectText "big-model"
    
    ; Set default selection
    ${NSD_SetText} $ModelSelectText "small-model"
    
    ; Show the dialog
    nsDialogs::Show
FunctionEnd

Function ModelSelectLeave
    ; Get selected model
    ${NSD_GetText} $ModelSelectText $ModelSelectState
    StrCpy $0 $ModelSelectState
FunctionEnd

; Custom page for license agreement
Page custom LicensePage LicenseLeave
Function LicensePage
    nsDialogs::Create 1018
    Pop $LicenseDialog
    
    ${If} $LicenseDialog == error
        Abort
    ${EndIf}
    
    ; Create license checkbox
    ${NSD_CreateCheckbox} 0 0 100% 12u "I agree to the license terms"
    Pop $LicenseText
    
    ; Show the dialog
    nsDialogs::Show
FunctionEnd

Function LicenseLeave
    ; Get license agreement state
    ${NSD_GetState} $LicenseText $LicenseState
    ${IfThen} $LicenseState == ${BST_UNCHECKED} ${|} Abort ${|}
FunctionEnd

; Custom page for telemetry consent
Page custom TelemetryPage TelemetryLeave
Function TelemetryPage
    nsDialogs::Create 1018
    Pop $TelemetryDialog
    
    ${If} $TelemetryDialog == error
        Abort
    ${EndIf}
    
    ; Create telemetry checkbox
    ${NSD_CreateCheckbox} 0 0 100% 12u "I agree to send telemetry data"
    Pop $TelemetryText
    
    ; Show the dialog
    nsDialogs::Show
FunctionEnd

Function TelemetryLeave
    ; Get telemetry consent state
    ${NSD_GetState} $TelemetryText $TelemetryState
    ${IfThen} $TelemetryState == ${BST_UNCHECKED} ${|} Abort {|}
FunctionEnd

; Custom page for Vulkan SDK check
Page custom VulkanPage VulkanLeave
Function VulkanPage
    nsDialogs::Create 1018
    Pop $VulkanDialog
    
    ${If} $VulkanDialog == error
        Abort
    ${EndIf}
    
    ; Check if Vulkan is installed
    StrCpy $0 "vulkan-1.dll"
    ${IfNot} ${FileExists} "$WINDIR\System32\$0"
        ; Create Vulkan checkbox
        ${NSD_CreateCheckbox} 0 0 100% 12u "Download and install Vulkan SDK"
        Pop $VulkanText
        
        ; Show the dialog
        nsDialogs::Show
    ${EndIf}
FunctionEnd

Function VulkanLeave
    ; Get Vulkan installation state
    ${NSD_GetState} $VulkanText $VulkanState
    ${IfThen} $VulkanState == ${BST_CHECKED} ${|} 
        ; Download and install Vulkan SDK
        nsisdl::download "https://vulkan-sdk.org/download/vulkan-sdk-win-latest.exe" "$TEMP\vulkan-sdk.exe"
        Pop $0
        ${If} $0 == "success"
            ExecWait '"$TEMP\vulkan-sdk.exe" /S'
        ${EndIf}
    {|}
FunctionEnd