import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

GridLayout {
    id: mainPageButtons
    anchors.fill: parent
    rows: 3; columns: 3
    columnSpacing: Kirigami.Units.largeSpacing
    rowSpacing: Kirigami.Units.smallSpacing

    // --- Column 1: Documentation ---
    Kirigami.Heading {
        level: 2
        text: documentationHeading
        Layout.alignment: Qt.AlignHCenter
    }
    Kirigami.Heading {
        level: 2
        text: supportHeading
        Layout.alignment: Qt.AlignHCenter
    }
    Kirigami.Heading {
        level: 2
        text: projectHeading
        Layout.alignment: Qt.AlignHCenter
    }

    Button {
        text: readmeButton
        onClicked: rustModel.showPage("readme")
        Layout.preferredWidth: 200; Layout.preferredHeight: 39
        Layout.fillWidth: true
    }
    Button {
        text: forumButton
        icon.name: "external-link"
        //tooltip: rustModel.webResourceTooltip
        onClicked: rustModel.openLink(forumUrl)
        Layout.preferredWidth: 200; Layout.preferredHeight: 39
        Layout.fillWidth: true
    }
    Button {
        text: involvedButton
        onClicked: rustModel.showPage("involved")
        Layout.preferredWidth: 200; Layout.preferredHeight: 39
        Layout.fillWidth: true
    }

    Button {
        text: releaseButton
        onClicked: rustModel.showPage("release")
        Layout.preferredWidth: 200; Layout.preferredHeight: 39
        Layout.fillWidth: true
    }
    Button {
        text: softwareButton
        icon.name: "external-link"
        //tooltip: rustModel.webResourceTooltip
        onClicked: rustModel.openLink(softwareUrl)
        Layout.preferredWidth: 200; Layout.preferredHeight: 39
        Layout.fillWidth: true
    }
    Button {
       text: developmentButton
       icon.name: "external-link"
       //tooltip: rustModel.webResourceTooltip
       onClicked: rustModel.openLink(developmentUrl)
       Layout.preferredWidth: 200; Layout.preferredHeight: 39
       Layout.fillWidth: true
    }

    Button {
        text: wikiButton
        icon.name: "external-link"
        //tooltip: rustModel.webResourceTooltip
        onClicked: rustModel.openLink(wikiUrl)
        Layout.preferredWidth: 200; Layout.preferredHeight: 39
        Layout.fillWidth: true
    }
    Rectangle { color: "transparent"; Layout.preferredWidth: 200; Layout.preferredHeight: 39 }
    Button {
        text: donateButton
        icon.name: "external-link"
        //tooltip: rustModel.webResourceTooltip
        onClicked: rustModel.openLink(donateUrl)
        Layout.preferredWidth: 200; Layout.preferredHeight: 39
        Layout.fillWidth: true
    }

    // --- Column 2: Support/Installation ---


    //RowLayout {
    //    Layout.row: 3
    //    Layout.column: 1
    //    Layout.alignment: Qt.AlignHCenter
    //    spacing: Kirigami.Units.smallSpacing
    //    IconButton { iconName: "telegram"; onClicked: rustModel.openLink(telegramUrl) }
    //    IconButton { iconName: "discord"; onClicked: rustModel.openLink(discordUrl) }
    //    IconButton { iconName: "reddit"; onClicked: rustModel.openLink(redditUrl) }
    //}

    // --- Installation Section (Conditional) ---
    //Kirigami.Heading {
    //    visible: installerVisible
    //    Layout.columnSpan: 1
    //    level: 3
    //    text: installationHeading
    //}
    //Button {
    //    visible: installerVisible
    //    text: launchInstallerButton
    //    onClicked: rustModel.launchInstaller()
    //}

    // --- Column 3: Project ---
}
