import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import org.kde.kirigami as Kirigami

Kirigami.Action {
    property string iconName
    text: iconName

    icon.name: iconName
}
