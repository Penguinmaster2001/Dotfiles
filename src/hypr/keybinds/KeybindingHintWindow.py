import gi
gi.require_version("Gtk", "3.0")
from gi.repository import Gtk

symbolDict = {
    "$mainMod" : "Super"
}


appBinds = [
    (["$mainMod"], "E", "nemo"),
    (["$mainMod"], "R", "runner"),
    # (["$mainMod"], "T", ""),
    # (["$mainMod"], "Y", ""),
    # (["$mainMod"], "U", "Math122"),
    # (["$mainMod"], "I", "Math101"),
    # (["$mainMod"], "O", "ATWP"),
    # (["$mainMod"], "P", "CSC115"),


    (["$mainMod"], "A", "VSCode"),
    (["$mainMod"], "S", "Gedit"),
    (["$mainMod"], "D", "Calculator"),
    # (["$mainMod"], "F", "")

    # (["$mainMod"], "G", "Pearson"),
    # (["$mainMod"], "H", "Brightspace"),
    # (["$mainMod"], "j", "Wikipedia"),
    # (["$mainMod"], "K", "Desmos"),
    (["$mainMod"], "L", "Firefox")
]


windowBinds = [
    (["$mainMod"], "E", "nemo"),
    (["$mainMod"], "R", "runner"),
    # (["$mainMod"], "T", ""),
    # (["$mainMod"], "Y", ""),
    # (["$mainMod"], "U", "Math122"),
    # (["$mainMod"], "I", "Math101"),
    # (["$mainMod"], "O", "ATWP"),
    # (["$mainMod"], "P", "CSC115"),


    (["$mainMod"], "A", "VSCode"),
    (["$mainMod"], "S", "Gedit"),
    (["$mainMod"], "D", "Calculator"),
    # (["$mainMod"], "F", "")

    # (["$mainMod"], "G", "Pearson"),
    # (["$mainMod"], "H", "Brightspace"),
    # (["$mainMod"], "j", "Wikipedia"),
    # (["$mainMod"], "K", "Desmos"),
    (["$mainMod"], "L", "Firefox")
]

categories = ["Apps", "Window", "Workspace", "System"]
bindingLists = [appBinds, windowBinds, appBinds, appBinds]

class MyWindow(Gtk.Window):
    def __init__(self):
        super().__init__(title="Keybinds")
        width = 100
        height = 2

        self.grid = Gtk.Grid()

        for i in range(len(categories)):
            newGrid = Gtk.Grid()

            bindingList = bindingLists[i]
            for j in range(len(bindingList)):
                bindings = bindingList[j]
                mod = symbolDict[bindings[0][0]]
                key = bindings[1]
                action = bindings[2]

                accStr = mod + " + " + key + " : " + action

                newLabel = Gtk.Label(label=accStr)
                newGrid.attach(newLabel, 0, j * height, width, height)
            
            newFrame = Gtk.Frame(child=newGrid, label=categories[i])
            
            self.grid.attach(newFrame, i * width, 0, width, height)


        # self.grid.add()
        # for frame in self.frameList:
        #     self.grid.add(frame)

        # self.winL = Gtk.Label(label="hello")
        # self.add(self.winL)

        # self.VSep = Gtk.separator()

        self.add(self.grid)


win = MyWindow()
win.connect("destroy", Gtk.main_quit)
win.show_all()
Gtk.main()
