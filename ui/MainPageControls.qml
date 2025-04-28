import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

RowLayout {
    id: mainPageControls
    //Layout.fillWidth: true
    //Layout.fillHeight: true
    //Layout.fillWidth: true

    anchors.fill: parent
    spacing: Kirigami.Units.mediumSpacing
    //horizontalItemAlignment: Grid.AlignHCenter
    //verticalItemAlignment: Grid.AlignVCenter
    //leftPadding: 10
    //rightPadding: 10

    ComboBox {
        id: languageCombo
        model: rustModel.localeModel
        textRole: "name"
        valueRole: "id"

        Layout.preferredHeight: 39
        currentIndex: 1//rustModel.localeModel.findIndex(function(item){ return item.id === rustModel.currentLocaleId })

        onActivated: rustModel.setLocale(currentValue)
    }

    Row { Layout.fillWidth: true } // Spacer

    RowLayout {
        Layout.alignment: Qt.AlignVCenter
        uniformCellSizes: true
        spacing: 2
        //spacing: Kirigami.Units.smallSpacing
        IconButton { iconName: "telegram"; onClicked: rustModel.openLink(telegramUrl) }
        IconButton { iconName: "discord"; onClicked: rustModel.openLink(discordUrl) }
        IconButton { iconName: "reddit"; onClicked: rustModel.openLink(redditUrl) }
    }

    Row { Layout.fillWidth: true } // Spacer

    Label { text: launchAtStartLabel }
    Switch {
        id: autostartSwitch
        //checked: autostartEnabled
        onCheckedChanged: rustModel.toggleAutostart()
    }

    component IconButton: Button {
        property string iconName

        Kirigami.Action {
            text: iconName
            icon.name: iconName
        }
    }
    function createIconButton(iconName, url) {
        var component = iconButtonComponent.createObject(homePage, {"iconName": iconName});
        component.clicked.connect(function() { rustModel.openLink(url); });
        return component;
    }
}
