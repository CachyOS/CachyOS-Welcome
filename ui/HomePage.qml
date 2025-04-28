import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "."

Kirigami.Page {
    id: homePage
    title: welcomeTitle
    anchors.fill: parent

    ColumnLayout {
        id: homePageLayout
        anchors.fill: parent
        spacing: Kirigami.Units.largeSpacing

        Kirigami.Heading {
            Layout.alignment: Qt.AlignHCenter
            level: 1
            text: welcomeTitle
        }

        /*Image {
            Layout.alignment: Qt.AlignHCenter
            sourceSize.height: Kirigami.Units.gridUnit * 8
            fillMode: Image.PreserveAspectFit
            source: "document-open"
        }*/

        Label {
            Layout.preferredHeight: 50
            Layout.fillWidth: true
            wrapMode: Label.WordWrap
            text: welcomeMessage
        }
        //Item { Layout.fillWidth: true } // Spacer

        Item {
            //color: "red"
            //Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.margins: 10

            MainPageButtons { }
        }
        //Item { Layout.fillWidth: true } // Spacer
        Rectangle { color: "transparent"; Layout.preferredHeight: 30 }

        // --- Bottom Controls ---
        Item {
            //color: "blue"
            Layout.fillWidth: true
            Layout.preferredHeight: 50
            Layout.margins: 10

            MainPageControls { }
        }
    }
}
