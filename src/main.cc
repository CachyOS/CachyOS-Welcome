#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <unordered_map>

#include <fmt/core.h>
#include <gtkmm.h>
#include <json.hpp>

namespace fs = std::filesystem;

Glib::RefPtr<Gtk::Application> g_app;
Glib::RefPtr<Gtk::Builder> g_refGlade;
Gtk::Window* g_refWindow;
nlohmann::json preferences;

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

void on_action_clicked(GtkWidget* widget) {
    const auto& name = gtk_widget_get_name(widget);
    if (strncmp(name, "install", 7) == 0) {
        fmt::print("install\n");
        return;
    } else if (strncmp(name, "autostart", 9) == 0) {
        //const auto& action = GTK_ACTION(widget);
        //set_autostart(action->get_active());
        fmt::print("autostart\n");

        return;
    }

    Gtk::AboutDialog* dialog;
    g_refGlade->get_widget<Gtk::AboutDialog>("aboutdialog", std::ref(dialog));
    dialog->set_decorated(false);
    dialog->run();
    dialog->hide();
}

void on_btn_clicked(GtkWidget* widget) {
    const auto& name = gtk_widget_get_name(widget);
    Gtk::Stack* stack;
    g_refGlade->get_widget<Gtk::Stack>("stack", std::ref(stack));
    stack->set_visible_child(fmt::format("{}page", name).c_str());
}

void on_link_clicked(GtkWidget* widget) {
    const auto& name      = gtk_widget_get_name(widget);
    const std::string uri = preferences["urls"][name];
    gtk_show_uri_on_window(nullptr, uri.c_str(), GDK_CURRENT_TIME, nullptr);
}

void on_delete_window(GtkWidget* widget) {
    g_app->quit();
}

nlohmann::json read_json(const std::string& path) {
    // read a JSON file
    std::ifstream i(path);
    nlohmann::json j;
    i >> j;

    return j;
}

int main(int argc, char** argv) {
    g_app       = Gtk::Application::create();
    auto screen = Gdk::Screen::get_default();

    bool is_dev = false;
    if (argc > 1 && (strncmp(argv[1], "--dev", 5) == 0)) {
        is_dev = true;
    }

    // Load preferences
    if (is_dev) {
        preferences                 = read_json("data/preferences.json");
        preferences["data_path"]    = "data/";
        preferences["desktop_path"] = (fs::current_path() / "cachyos-hello.desktop").string();
        preferences["locale_path"]  = "locale/";
        preferences["ui_path"]      = "ui/cachyos-hello.glade";
        preferences["style_path"]   = "ui/style.css";
    } else {
        preferences = read_json("/usr/share/cachyos-hello/data/preferences.json");
    }

    // Get saved infos
    //const auto& save = read_json(preferences["save_path"]);

    // Import Css
    auto provider = Gtk::CssProvider::create();
    provider->load_from_path(preferences["style_path"]);
    Gtk::StyleContext::add_provider_for_screen(screen, provider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Init window
    g_refGlade = Gtk::Builder::create_from_file(preferences["ui_path"]);
    gtk_builder_add_callback_symbol(g_refGlade->gobj(), "on_action_clicked", G_CALLBACK(on_action_clicked));
    gtk_builder_add_callback_symbol(g_refGlade->gobj(), "on_btn_clicked", G_CALLBACK(on_btn_clicked));
    gtk_builder_add_callback_symbol(g_refGlade->gobj(), "on_link_clicked", G_CALLBACK(on_link_clicked));
    gtk_builder_add_callback_symbol(g_refGlade->gobj(), "on_delete_window", G_CALLBACK(on_delete_window));
    gtk_builder_connect_signals(g_refGlade->gobj(), nullptr);
    g_refGlade->get_widget<Gtk::Window>("window", std::ref(g_refWindow));

    // Subtitle of headerbar
    Gtk::HeaderBar* header;
    g_refGlade->get_widget<Gtk::HeaderBar>("headerbar", std::ref(header));
    const auto& lsb_info = get_lsb_infos();
    header->set_subtitle(lsb_info[0] + " " + lsb_info[1]);

    // Load images
    if (fs::is_regular_file(preferences["logo_path"])) {
        const auto& logo = Gdk::Pixbuf::create_from_file(preferences["logo_path"]);
        g_refWindow->set_icon(logo);

        Gtk::Image* image;
        g_refGlade->get_widget<Gtk::Image>("distriblogo", std::ref(image));
        image->set(logo);

        Gtk::AboutDialog* dialog;
        g_refGlade->get_widget<Gtk::AboutDialog>("aboutdialog", std::ref(dialog));
        dialog->set_logo(logo);
    }

    Gtk::Box* social_box;
    g_refGlade->get_widget<Gtk::Box>("social", std::ref(social_box));
    for (const auto& btn : social_box->get_children()) {
        const auto& name      = btn->get_name();
        const auto& icon_path = fmt::format("{}img/{}.png", preferences["data_path"], name.c_str());
        Gtk::Image* image;
        g_refGlade->get_widget<Gtk::Image>(name, image);
        image->set(icon_path);
    }

    Gtk::Grid* homepage_grid;
    g_refGlade->get_widget<Gtk::Grid>("homepage", std::ref(homepage_grid));
    for (const auto& widget : homepage_grid->get_children()) {
        if (!G_TYPE_CHECK_INSTANCE_TYPE(widget->gobj(), GTK_TYPE_BUTTON)) {
            continue;
        }
        const auto& casted_widget = GTK_BUTTON(widget->gobj());
        if (gtk_button_get_image_position(casted_widget) != GtkPositionType::GTK_POS_RIGHT) {
            continue;
        }

        Gtk::Image image(fmt::format("{}/img/external-link.png", preferences["data_path"]));
        image.set_margin_start(2);
        gtk_button_set_image(casted_widget, (GtkWidget*)image.gobj());
    }

    // Create pages
    const auto& pages = fmt::format("{}/pages/{}", preferences["data_path"], preferences["default_locale"]);

    for (const auto& page : fs::directory_iterator(pages)) {
        Gtk::ScrolledWindow scrolled_window;
        Gtk::Viewport viewport(Gtk::Adjustment::create(1, 1, 1), Gtk::Adjustment::create(1, 1, 1));
        Gtk::Label label;
        label.set_line_wrap(true);
        Gtk::Image image(Gtk::Stock::GO_BACK, Gtk::ICON_SIZE_BUTTON);
        Gtk::Button backBtn;
        backBtn.set_image(image);
        backBtn.set_name("home");
        backBtn.signal_clicked().connect(sigc::bind(sigc::ptr_fun(on_btn_clicked), (GtkWidget*)&backBtn));

        Gtk::Grid grid;
        grid.attach(backBtn, 0, 1, 1, 1);
        grid.attach(label, 1, 2, 1, 1);
        viewport.add(grid);
        scrolled_window.add(viewport);
        scrolled_window.show_all();

        Gtk::Stack* stack;
        g_refGlade->get_widget<Gtk::Stack>("stack", std::ref(stack));
        stack->add(scrolled_window, page.path().stem().string() + "page");
    }

    // Shows the window and returns when it is closed.
    return g_app->run(*g_refWindow);
}
