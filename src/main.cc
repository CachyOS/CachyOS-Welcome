#include <fstream>
#include <iostream>
#include <iterator>
#include <unordered_map>

#include <gtkmm/application.h>
#include <gtkmm/builder.h>
#include <gtkmm/button.h>
#include <gtkmm/cssprovider.h>
#include <gtkmm/headerbar.h>
#include <gtkmm/scrolledwindow.h>
#include <gtkmm/window.h>
#include <json.hpp>

Glib::RefPtr<Gtk::Builder> g_refGlade;
Gtk::Window* g_refWindow;

/*
class HelloWorld : public Gtk::Window {

 public:
    HelloWorld() : m_button("Hello World") {
        // Sets the border width of the window.
        set_border_width(10);

        // When the button receives the "clicked" signal, it will call the
        // on_button_clicked() method defined below.
        m_button.signal_clicked().connect(
            sigc::mem_fun(*this, &HelloWorld::on_button_clicked));

        m_builder = Gtk::Builder::create_from_file("ui/cachyos-hello.glade");

        // This packs the button into the Window (a container).
        add(m_button);

        // The final step is to display this newly created widget...
        m_button.show();
    }
    virtual ~HelloWorld() = default;

 protected:
    // Signal handlers:
    void on_button_clicked() { std::cout << "Hello World\n"; }

    // Member widgets:
    Gtk::Button m_button;
    Gtk::Builder m_builder;
};*/

std::pair<std::string, std::string> tokenize(std::string str, std::string delim) {
    int start = 0;
    int end   = str.find(delim);
    std::string key{};
    while (end != -1) {
        key   = str.substr(start, end - start);
        start = end + delim.size();
        end   = str.find(delim, start);
    }
    return {key, str.substr(start, end - start)};
}

std::size_t replace_all(std::string& inout, std::string_view what, std::string_view with) {
    std::size_t count{};
    for (std::string::size_type pos{};
         inout.npos != (pos = inout.find(what.data(), pos, what.length()));
         pos += with.length(), ++count) {
        inout.replace(pos, what.length(), with.data(), with.length());
    }
    return count;
}

std::size_t remove_all(std::string& inout, std::string_view what) {
    return replace_all(inout, what, "");
}

// Read informations from the lsb-release file.
//
// @Returns args from lsb-release file
std::array<std::string, 2> get_lsb_infos() {
    std::unordered_map<std::string, std::string> lsb{};

    try {
        std::ifstream lsb_release("/etc/lsb-release");
        std::string line;
        while (std::getline(lsb_release, line)) {
            if (line.find("=") != std::string::npos) {
                auto var = tokenize(line, "=");
                remove_all(var.first, "DISTRIB_");
                remove_all(var.second, "\"");
                lsb[var.first] = var.second;
            }
        }
    } catch (const std::exception& e) {
        std::cerr << e.what() << '\n';
        return {"not CachyOS", "0.0"};
    }
    return {lsb["ID"], lsb["RELEASE"]};
}

nlohmann::json read_json(const std::string& path) {
    // read a JSON file
    std::ifstream i(path);
    nlohmann::json j;
    i >> j;

    return j;
}

int main(int argc, char* argv[]) {
    auto app = Gtk::Application::create(argc, argv, "org.gtkmm.example");

    // Load preferences
    const auto& preferences = read_json("data/preferences.json");

    // Get saved infos
    //const auto& save = read_json(preferences["save_path"]);

    // Import Css
    //Gtk::CssProvider* provider = new Gtk::CssProvider();
    //provider.load_from_path("ui/style.css");

    g_refGlade = Gtk::Builder::create_from_file("ui/cachyos-hello.glade");
    g_refGlade->get_widget<Gtk::Window>("window", std::ref(g_refWindow));

    // Subtitle of headerbar
    Gtk::HeaderBar* header;
    g_refGlade->get_widget<Gtk::HeaderBar>("headerbar", std::ref(header));
    const auto& lsb_info = get_lsb_infos();
    header->set_subtitle(lsb_info[0] + " " + lsb_info[1]);

    //HelloWorld helloworld;

    // Shows the window and returns when it is closed.
    return app->run(*g_refWindow);
}
