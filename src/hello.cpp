#include "hello.hpp"
#include "helper.hpp"

#include <libintl.h>

#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <unordered_map>

#include <fmt/core.h>

namespace fs = std::filesystem;

namespace {
Hello* g_refHello;

std::string fix_path(std::string&& path) noexcept {
    if (path[0] != '~') {
        return path;
    }
    replace_all(path, "~", Glib::get_home_dir().c_str());
    return path;
}

nlohmann::json read_json(const std::string_view& path) {
    // read a JSON file
    std::ifstream i(fix_path(path.data()));
    nlohmann::json j;
    i >> j;

    return j;
}

void write_json(const std::string_view& path, const nlohmann::json& content) {
    // write data to JSON file
    std::ofstream o(fix_path(path.data()));
    o << content << '\n';
}

// Read information from the lsb-release file.
//
// @Returns args from lsb-release file
std::array<std::string, 2> get_lsb_infos() {
    std::unordered_map<std::string, std::string> lsb{};

    try {
        std::ifstream lsb_release("/etc/lsb-release");
        std::string line;
        while (std::getline(lsb_release, line)) {
            if (line.find('=') != std::string::npos) {
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
}  // namespace

Hello::Hello(int argc, char** argv) {
    set_title("CachyOS Hello");
    set_border_width(6);
    if (argc > 1 && (strncmp(argv[1], "--dev", 5) == 0)) {
        m_dev = true;
    }

    g_refHello = this;

    auto screen = Gdk::Screen::get_default();

    // Load preferences
    if (m_dev) {
        m_preferences                 = read_json("data/preferences.json");
        m_preferences["data_path"]    = "data/";
        m_preferences["desktop_path"] = fmt::format("{}/{}.desktop", fs::current_path().string(), m_app);
        m_preferences["locale_path"]  = "locale/";
        m_preferences["ui_path"]      = fmt::format("ui/{}.glade", m_app);
        m_preferences["style_path"]   = "ui/style.css";
    } else {
        m_preferences = read_json(fmt::format("/usr/share/{}/data/preferences.json", m_app));
    }

    // Get saved infos
    const auto& save_path = fix_path(m_preferences["save_path"]);
    m_save                = (!fs::exists(save_path)) ? nlohmann::json({{"locale", ""}}) : read_json(save_path);

    // Import Css
    auto provider = Gtk::CssProvider::create();
    provider->load_from_path(m_preferences["style_path"]);
    Gtk::StyleContext::add_provider_for_screen(screen, provider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Init window
    m_builder = Gtk::Builder::create_from_file(m_preferences["ui_path"]);
    gtk_builder_add_callback_symbol(m_builder->gobj(), "on_languages_changed", G_CALLBACK(on_languages_changed));
    gtk_builder_add_callback_symbol(m_builder->gobj(), "on_action_clicked", G_CALLBACK(on_action_clicked));
    gtk_builder_add_callback_symbol(m_builder->gobj(), "on_btn_clicked", G_CALLBACK(on_btn_clicked));
    gtk_builder_add_callback_symbol(m_builder->gobj(), "on_link_clicked", G_CALLBACK(on_link_clicked));
    gtk_builder_add_callback_symbol(m_builder->gobj(), "on_delete_window", G_CALLBACK(on_delete_window));
    gtk_builder_connect_signals(m_builder->gobj(), nullptr);
    Gtk::Window* ref_window;
    m_builder->get_widget("window", ref_window);
    gobject_ = reinterpret_cast<GObject*>(ref_window->gobj());

    // Subtitle of headerbar
    Gtk::HeaderBar* header;
    m_builder->get_widget("headerbar", header);
    const auto& lsb_info = get_lsb_infos();
    header->set_subtitle(lsb_info[0] + " " + lsb_info[1]);

    // Load images
    if (fs::is_regular_file(m_preferences["logo_path"])) {
        const auto& logo = Gdk::Pixbuf::create_from_file(m_preferences["logo_path"]);
        set_icon(logo);

        Gtk::Image* image;
        m_builder->get_widget("distriblogo", image);
        image->set(logo);

        Gtk::AboutDialog* dialog;
        m_builder->get_widget("aboutdialog", dialog);
        dialog->set_logo(logo);
    }

    Gtk::Box* social_box;
    m_builder->get_widget("social", social_box);
    for (const auto& btn : social_box->get_children()) {
        const auto& name      = btn->get_name();
        const auto& icon_path = fmt::format("{}img/{}.png", m_preferences["data_path"], name.c_str());
        Gtk::Image* image;
        m_builder->get_widget(name, image);
        image->set(icon_path);
    }

    Gtk::Grid* homepage_grid;
    m_builder->get_widget("homepage", homepage_grid);
    for (const auto& widget : homepage_grid->get_children()) {
        if (!G_TYPE_CHECK_INSTANCE_TYPE(widget->gobj(), GTK_TYPE_BUTTON)) {
            continue;
        }
        const auto& casted_widget = Glib::wrap(GTK_BUTTON(widget->gobj()));
        if (gtk_button_get_image_position(casted_widget->gobj()) != GtkPositionType::GTK_POS_RIGHT) {
            continue;
        }

        const auto& image_path = fmt::format("{}img/external-link.png", m_preferences["data_path"]);
        Gtk::Image image;
        image.set(image_path);
        image.set_margin_start(2);
        casted_widget->set_image(image);
    }

    // Create pages
    m_pages = fmt::format("{}pages/{}", m_preferences["data_path"], m_preferences["default_locale"]);

    for (const auto& page : fs::directory_iterator(m_pages)) {
        auto* scrolled_window = gtk_scrolled_window_new(nullptr, nullptr);
        auto* viewport        = gtk_viewport_new(nullptr, nullptr);
        gtk_container_set_border_width(GTK_CONTAINER(viewport), 10);
        auto* label = gtk_label_new(nullptr);
        gtk_label_set_line_wrap(GTK_LABEL(label), true);
        auto* image   = gtk_image_new_from_icon_name("go-previous", GTK_ICON_SIZE_BUTTON);
        auto* backBtn = gtk_button_new();
        gtk_button_set_image(GTK_BUTTON(backBtn), image);
        gtk_widget_set_name(backBtn, "home");
        g_signal_connect(backBtn, "clicked", G_CALLBACK(&on_btn_clicked), nullptr);

        auto* grid = GTK_GRID(gtk_grid_new());
        gtk_grid_attach(grid, backBtn, 0, 1, 1, 1);
        gtk_grid_attach(grid, label, 1, 2, 1, 1);
        gtk_container_add(GTK_CONTAINER(viewport), GTK_WIDGET(grid));
        gtk_container_add(GTK_CONTAINER(scrolled_window), GTK_WIDGET(viewport));
        gtk_widget_show_all(scrolled_window);

        Glib::RefPtr<Glib::Object> stack = m_builder->get_object("stack");
        const auto& child_name           = page.path().filename().string() + "page";
        gtk_stack_add_named(GTK_STACK(stack->gobj()), scrolled_window, child_name.c_str());
    }

    // Init translation
    const std::string& locale_path = m_preferences["locale_path"];
    bindtextdomain(m_app, locale_path.c_str());
    textdomain(m_app);
    Gtk::ComboBoxText* languages;
    m_builder->get_widget("languages", languages);
    languages->set_active_id(get_best_locale());

    // Set autostart switcher state
    m_autostart = fs::is_regular_file(fix_path(m_preferences["autostart_path"]));
    Gtk::Switch* autostart_switch;
    m_builder->get_widget("autostart", autostart_switch);
    autostart_switch->set_active(m_autostart);
}

/// Returns the best locale, based on user's preferences.
auto Hello::get_best_locale() const noexcept -> std::string {
    const auto& binary_path  = fmt::format("{}{}{}.mo", m_preferences["locale_path"], "{}/LC_MESSAGES/", m_app);
    const auto& saved_locale = fmt::vformat(binary_path, fmt::make_format_args(m_save["locale"]));
    if (fs::is_regular_file(saved_locale)) {
        return m_save["locale"];
    } else if (m_save["locale"] == m_preferences["default_locale"]) {
        return m_preferences["default_locale"];
    }

    const auto& locale_name = std::locale("").name();
    std::string sys_locale  = locale_name.substr(0, locale_name.find('.'));
    const auto& user_locale = fmt::vformat(binary_path, fmt::make_format_args(sys_locale));
    const auto& two_letters = sys_locale.substr(0, 2);

    // If user's locale is supported
    if (fs::is_regular_file(user_locale)) {
        if (sys_locale.find('_') != std::string::npos) {
            replace_all(sys_locale, "_", "-");
        }
        return sys_locale;
    }
    // If two first letters of user's locale is supported (ex: en_US -> en)
    else if (fs::is_regular_file(fmt::vformat(binary_path, fmt::make_format_args(two_letters)))) {
        return two_letters;
    }

    return m_preferences["default_locale"];
}

/// Sets locale of ui and pages.
void Hello::set_locale(const std::string_view& use_locale) noexcept {
    fmt::print(
        "┌{0:─^{2}}┐\n"
        "│{1: ^{2}}│\n"
        "└{0:─^{2}}┘\n",
        "", fmt::format("Locale changed to {}", use_locale), 40);

    bind_textdomain_codeset(m_app, use_locale.data());

    m_save["locale"] = use_locale;

    // Real-time locale changing
    /* clang-format off */
    nlohmann::json elts = {
        {"comments", {"aboutdialog"}},
        {"label", {
            "autostartlabel",
            "development",
            "discover",
            "donate",
            "firstcategory",
            "forum",
            "install",
            "installlabel",
            "involved",
            "mailling",
            "readme",
            "release",
            "secondcategory",
            "thirdcategory",
            "welcomelabel",
            "welcometitle",
            "wiki"}
        },
        {"tooltip_text", {
            "about",
            "development",
            "discover",
            "donate",
            "forum",
            "mailling",
            "wiki"}
        }};
    /* clang-format on */

    for (const auto& method : elts.items()) {
        if (!m_default_texts.contains(method.key())) {
            m_default_texts[method.key()] = {};
        }
        for (const auto& elt : elts[method.key()].items()) {
            const std::string& elt_value = elt.value();
            if (!m_default_texts[method.key()].contains(elt_value)) {
                Gtk::Widget* item;
                m_builder->get_widget(elt_value, item);
                gchar* item_buf;
                g_object_get(G_OBJECT(item->gobj()), method.key().c_str(), &item_buf, nullptr);
                m_default_texts[method.key()][elt_value] = item_buf;
                g_free(item_buf);
            }
        }
    }

    // Change content of pages
    for (const auto& page : fs::directory_iterator(m_pages)) {
        Gtk::Stack* stack;
        m_builder->get_widget("stack", stack);
        const auto& child = stack->get_child_by_name((page.path().filename().string() + "page").c_str());
        if (child == nullptr) {
            fmt::print(stderr, "child not found\n");
            continue;
        }
        const auto& first_child  = reinterpret_cast<Gtk::Container*>(child)->get_children();
        const auto& second_child = reinterpret_cast<Gtk::Container*>(first_child[0])->get_children();
        const auto& third_child  = reinterpret_cast<Gtk::Container*>(second_child[0])->get_children();

        const auto& label = reinterpret_cast<Gtk::Label*>(third_child[0]);
        label->set_markup(get_page(page.path().filename().string()));
    }
}

auto Hello::get_page(const std::string& name) const noexcept -> std::string {
    auto filename = fmt::format("{}pages/{}/{}", m_preferences["data_path"], m_save["locale"], name);
    if (!fs::is_regular_file(filename)) {
        filename = fmt::format("{}pages/{}/{}", m_preferences["data_path"], m_preferences["default_locale"], name);
    }

    return read_whole_file(filename);
}

// Handlers
void Hello::on_languages_changed(GtkComboBox* combobox) noexcept {
    const auto& active_id = gtk_combo_box_get_active_id(combobox);
    g_refHello->set_locale(active_id);
}

void Hello::on_action_clicked(GtkWidget* widget) noexcept {
    const auto& name = gtk_widget_get_name(widget);
    if (strncmp(name, "install", 7) == 0) {
        fmt::print("install\n");
        return;
    } else if (strncmp(name, "autostart", 9) == 0) {
        const auto& action = Glib::wrap(GTK_SWITCH(widget));
        //set_autostart(action->get_active());
        fmt::print("autostart\n");

        return;
    }

    Gtk::AboutDialog* dialog;
    g_refHello->m_builder->get_widget("aboutdialog", dialog);
    dialog->set_decorated(false);
    dialog->run();
    dialog->hide();
}

void Hello::on_btn_clicked(GtkWidget* widget) noexcept {
    const auto& name = gtk_widget_get_name(widget);
    Gtk::Stack* stack;
    g_refHello->m_builder->get_widget("stack", stack);
    stack->set_visible_child(fmt::format("{}page", name).c_str());
}
void Hello::on_link_clicked(GtkWidget* widget) noexcept {
    const auto& name      = gtk_widget_get_name(widget);
    const std::string uri = g_refHello->m_preferences["urls"][name];
    gtk_show_uri_on_window(nullptr, uri.c_str(), GDK_CURRENT_TIME, nullptr);
}
void Hello::on_delete_window(GtkWidget* /*widget*/) noexcept {
    write_json(g_refHello->m_preferences["save_path"].get<std::string>(), g_refHello->m_save);
    const auto& application = g_refHello->get_application();
    application->quit();
}