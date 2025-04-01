!macro NSIS_HOOK_PREINSTALL
  MessageBox MB_OK "This is pre-release software. If you encounter an issue, please provide a report at https://github.com/SurajSSingh/SuperMouseAI/issues"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  MessageBox MB_OK "Super Mouse AI is installed!"
  MessageBox MB_OK "Please make sure you have installed the Vulkan Runtime (available at https://vulkan.lunarg.com/sdk/home)"

!macroend

!macro NSIS_HOOK_PREUNINSTALL
  MessageBox MB_OK "Starting uninstallation process"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  MessageBox MB_OK "Super Mouse AI is now un-installed!"
!macroend