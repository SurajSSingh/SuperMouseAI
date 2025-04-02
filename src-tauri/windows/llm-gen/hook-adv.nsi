; NSIS_HOOK_PREINSTALL: Runs before copying files
!macro NSIS_HOOK_PREINSTALL
    ; Initialize variables
    Var DownloadProgress
    Var DownloadTotal
    Var DownloadReceived
    Var DownloadError
    Var ModelValidationResult
    
    ; Download GGML model
    ClearErrors
    StrCpy $DownloadTotal 0
    StrCpy $DownloadReceived 0
    StrCpy $DownloadError 0
    
    ; Show download progress
    nsisdl::download_quiet "${MODELURL}" "$PLUGINSDIR\${MODELFILE}"
    Pop $DownloadError
    
    ; Handle download errors
    ${If} $DownloadError != "success"
        MessageBox MB_ICONERROR "$(downloadFailed)" /SD IDABORT
        Abort
    ${EndIf}
    
    ; Validate downloaded model
    ClearErrors
    ExecWait "${MODELVALIDATIONCMD} $PLUGINSDIR\${MODELFILE}" $ModelValidationResult
    ${If} $ModelValidationResult != 0
        MessageBox MB_ICONERROR "$(invalidModel)" /SD IDABORT
        Abort
    ${EndIf}
    
    ; Copy model to destination
    CopyFiles /SILENT "$PLUGINSDIR\${MODELFILE}" "${MODELDEST}"
!macroend

; NSIS_HOOK_POSTINSTALL: Runs after installation
!macro NSIS_HOOK_POSTINSTALL
    ; Verify installation
    ClearErrors
    ReadRegStr $0 HKLM "Software\${MANUFACTURER}\${PRODUCTNAME}" "Installed"
    ${If} $0 != "1"
        WriteRegStr HKLM "Software\${MANUFACTURER}\${PRODUCTNAME}" "Installed" "1"
        WriteRegStr HKLM "Software\${MANUFACTURER}\${PRODUCTNAME}" "Version" "${VERSION}"
    ${EndIf}
    
    ; Create desktop shortcut if requested
    ${GetOptions} $CMDLINE "/D" $R0
    ${IfNot} ${Errors}
        CreateShortCut "$DESKTOP\${PRODUCTNAME}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe"
    ${EndIf}
!macroend

; NSIS_HOOK_PREUNINSTALL: Runs before uninstallation
!macro NSIS_HOOK_PREUNINSTALL
    ; Check if app is running
    !insertmacro CheckIfAppIsRunning
    
    ; Backup model file if configured
    ${If} ${FileExists} "${MODELDEST}"
        CopyFiles /SILENT "${MODELDEST}" "$PLUGINSDIR\${MODELFILE}.backup"
    ${EndIf}
!macroend

; NSIS_HOOK_POSTUNINSTALL: Runs after uninstallation
!macro NSIS_HOOK_POSTUNINSTALL
    ; Remove backup if it exists
    ${If} ${FileExists} "$PLUGINSDIR\${MODELFILE}.backup"
        Delete "$PLUGINSDIR\${MODELFILE}.backup"
    ${EndIf}
    
    ; Clean up registry entries
    DeleteRegKey HKLM "Software\${MANUFACTURER}\${PRODUCTNAME}"
!macroend