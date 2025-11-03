
import subprocess
import sys
import workspaceTools



if len(sys.argv) < 2:
    exit(0)



if not sys.argv[1].isdigit():
    exit(0)



move_right: bool = int(sys.argv[1]) > 0

current_workspace_result = workspaceTools.get_current_workspace()



if not current_workspace_result.is_success():
    exit(0)



current_workspace = current_workspace_result.get_value()

find_result = workspaceTools.find_next_binary_partition_workspace(current_workspace, move_right)



if find_result.is_success():
    subprocess.run(["hyprctl", "dispatch", "workspace", str(find_result.get_value())])

