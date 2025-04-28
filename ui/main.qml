import QtQuick 2.15
import QtQuick.Controls 2.15
import org.kde.kirigami as Kirigami

Kirigami.ApplicationWindow {
    id: root
    // Bind title to Rust model property
    title: windowTitle
    width: 800
    height: 600
    minimumWidth: 600
    minimumHeight: 500
    visible: true

    // Global drawer or other navigation could go here if desired

    /*
    // Global actions (e.g., About)
    globalDrawer: Kirigami.GlobalDrawer {
        modal: true
        isMenu: true

        actions: [
           Kirigami.Action {
               // Bind text and tooltip
               text: aboutActionText
               tooltip: aboutButtonTooltip
               icon.name: "help-about"
               onTriggered: rustModel.showAbout() // Call Rust method
           }
        ]
    }*/

    // Main content area using StackView for navigation
    pageStack.initialPage: homePageComponent

    Component.onCompleted: {
        console.log("QML main window loaded.");
        // Connect Rust signal for navigation
        /*rustModel.navigate.connect(function(pageId, title, content) {
            if (pageId === "home") {
                pageStack.pop(); // Assuming HomePage is the base
            } else {
                // Push InfoPage or specific page components later
                pageStack.push(infoPageComponent, { pageTitle: title, pageContent: content });
            }
        });*/
    }

    Item {
        //color: "black"
        id: homePageComponent
        height: parent.height
        width: parent.width
        HomePage { }
    }

    // Component for the InfoPage
    //Component {
    //    id: infoPageComponent
    //    InfoPage {}
    //}
}
