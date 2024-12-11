
import json
import subprocess



class workspace_result:

    def __init__(self, value: int, success: bool, error_message: str = ""):

        self.value = value
        self.success = success
        self.error_message = error_message


    def is_success(self) -> bool:

        return self.success


    def get_value(self) -> int:

        if not self.success:
            raise ValueError("Attempted to get value from an unsuccessful result")
        
        return self.value


    def get_error_message(self) -> str:

        if self.success:
            raise ValueError("Attempted to get error message from a successful result")
        
        return self.error_message
    


def get_current_workspace() -> workspace_result:

    active_workspace_result = subprocess.run(["hyprctl", "activeworkspace", "-j"], capture_output=True)

    if active_workspace_result.returncode != 0:
        
        return workspace_result(-1, False, f"`hyprctl activeworkspace -j` failed with result {active_workspace_result.returncode}. stderr: {active_workspace_result.stderr.decode()}")

    return workspace_result(int(json.loads(active_workspace_result.stdout)["id"]), True)



def find_next_binary_partition_workspace(current_workspace: int, search_right: bool) -> workspace_result:

    workspaces_result = subprocess.run(["hyprctl", "workspaces", "-j"], capture_output = True)

    if workspaces_result.returncode != 0:
        
        return workspace_result(-1, False, f"`hyprctl workspaces -j` failed with result {workspaces_result.returncode}. stderr: {workspaces_result.stderr.decode()}")


    workspaces = json.loads(workspaces_result.stdout)

    greatest_lower_workspace = 0
    least_higher_workspace = 2 ** 31

    for workspace in workspaces:
        id: int = workspace["id"]

        if id < current_workspace and id > greatest_lower_workspace:
            greatest_lower_workspace = id

        if id > current_workspace and id < least_higher_workspace:
            least_higher_workspace = id

    return workspace_result(int((current_workspace + (least_higher_workspace if search_right else greatest_lower_workspace)) / 2), True)
