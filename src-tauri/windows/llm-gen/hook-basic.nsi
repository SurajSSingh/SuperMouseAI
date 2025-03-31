; NSIS_HOOK_PREINSTALL: Basic download and validation
!macro NSIS_HOOK_PREINSTALL
    ; Download GGML model
    nsisdl::download_quiet "${MODELURL}" "$PLUGINSDIR\${MODELFILE}"
    Pop $0
    ${If} $0 != "success"
        MessageBox MB_ICONERROR "Download failed: $0" /SD IDABORT
        Abort
    ${EndIf}
    
    ; Basic validation
    ${If} ${FileExists} "$PLUGINSDIR\${MODELFILE}"
        CopyFiles "$PLUGINSDIR\${MODELFILE}" "${MODELDEST}"
    ${Else}
        MessageBox MB_ICONERROR "Model file not found" /SD IDABORT
        Abort
    ${EndIf}
!macroend

; NSIS_HOOK_POSTINSTALL: Basic verification
!macro NSIS_HOOK_POSTINSTALL
    ${If} ${FileExists} "${MODELDEST}"
        WriteRegStr HKLM "Software\${MANUFACTURER}\${PRODUCTNAME}" "ModelInstalled" "1"
    ${Else}
        MessageBox MB_ICONERROR "Model installation failed" /SD IDABORT
        Abort
    ${EndIf}
!macroend

; NSIS_HOOK_PREUNINSTALL: Basic cleanup
!macro NSIS_HOOK_PREUNINSTALL
    !insertmacro CheckIfAppIsRunning
!macroend

; NSIS_HOOK_POSTUNINSTALL: Basic registry cleanup
!macro NSIS_HOOK_POSTUNINSTALL
    DeleteRegKey HKLM "Software\${MANUFACTURER}\${PRODUCTNAME}"
!macroend