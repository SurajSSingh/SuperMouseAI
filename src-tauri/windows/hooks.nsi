; Adapted from <https://github.com/thewh1teagle/vibe/blob/main/desktop/src-tauri/vc_redist.nsh>

!addplugindir .\plugins

Section
; Check if Vulkan runtime is already installed
IfFileExists "$WINDIR\System32\vulkan-1.dll" file_found file_not_found
file_found:
    DetailPrint "Vulkan runtime found! Skipping installation."
    Goto end_of_vulkan
file_not_found:
    ; Define download URL and target path for Vulkan SDK installer
    StrCpy $0 "https://sdk.lunarg.com/sdk/download/latest/windows/vulkan_sdk.exe"
    StrCpy $1 "$TEMP\vulkan_sdk.exe"
    
    ; Download the Vulkan SDK installer
    DetailPrint "Downloading latest SDK from $0 ."
    inetc::get $0 $1
    Pop $0
    ${If} $0 == "success"
        DetailPrint "Vulkan SDK downloaded successfully"
    ${Else}
        DetailPrint "Vulkan SDK download failed"
        Call InstallFailed
        Abort "Vulkan SDK download failed, aborting installation"
    ${EndIf}

    ; Execute the downloaded installer silently
    ExecWait '"$1" --no-graphics --silent' $0
    ${If} $0 == 0
        DetailPrint "Vulkan SDK installation completed successfully"
    ${Else}
        DetailPrint "Vulkan SDK installation failed"
        Call InstallFailed
        Abort "Vulkan SDK installation failed, aborting process"
    ${EndIf}
end_of_vulkan:
SectionEnd


Function InstallFailed
    DetailPrint "Vulkan failed to download"
    ; Show a message box to inform the user
    MessageBox MB_OK|MB_ICONEXCLAMATION "Failed to download Vulkan. Please download and install it manually. Click OK to open the URL to download."
    ; Open the URL in the default browser
    ExecShell "open" "https://vulkan.lunarg.com/sdk/home#windows"
FunctionEnd

!macro NSIS_HOOK_PREINSTALL
  MessageBox MB_OK "This is pre-release software. If you encounter an issue, please provide a report on our GitHub repository page"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  MessageBox MB_OK "Super Mouse AI is installed!"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  MessageBox MB_OK "Starting uninstallation process"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  MessageBox MB_OK "Super Mouse AI is now un-installed!"
!macroend