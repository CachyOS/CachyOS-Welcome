import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import org.kde.kirigami as Kirigami

// Page for displaying content fetched from Rust (e.g., readme, release info)
Kirigami.Page {
    id: infoPage
    property alias pageContent: contentLabel.text

    header: Kirigami.Heading {
        id: header
        level: 1
        // Title set externally when page is pushed
    }

    Kirigami.ScrollablePage {
        Label {
            id: contentLabel
            textFormat: Text.MarkdownText // Assume content is Markdown
            wrapMode: Label.WordWrap
            // Content set externally when page is pushed
            Layout.fillWidth: true
            Layout.fillHeight: true // Or manage height appropriately
        }
    }
}
