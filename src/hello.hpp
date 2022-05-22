#ifndef HELLO_HPP_
#define HELLO_HPP_

#include <vector>

#include <gtkmm.h>
#include <json.hpp>

class Hello final : public Gtk::Window {
 public:
    Hello(int argc, char** argv);

    const std::vector<std::string>& get_loaded_units() const
    { return m_loaded_units; }
    const std::vector<std::string>& get_enabled_units() const
    { return m_enabled_units; }
    const std::vector<std::string>& get_global_loaded_units() const
    { return m_global_loaded_units; }
    const std::vector<std::string>& get_global_enabled_units() const
    { return m_global_enabled_units; }

    void load_enabled_units() noexcept;
    void load_global_enabled_units() noexcept;

 protected:
    // Handlers
    static void on_languages_changed(GtkComboBox* combobox) noexcept;
    static void on_action_clicked(GtkWidget* widget) noexcept;
    static void on_btn_clicked(GtkWidget* widget) noexcept;
    static void on_link_clicked(GtkWidget* widget) noexcept;
    static void on_delete_window(GtkWidget* /*widget*/) noexcept;

 private:
    static constexpr auto m_app = "cachyos-hello";
    bool m_dev{};
    bool m_autostart{};

    std::string m_pages;
    Glib::RefPtr<Gtk::Builder> m_builder;
    nlohmann::json m_preferences;
    nlohmann::json m_save;
    nlohmann::json m_default_texts;
    std::vector<std::string> m_loaded_units;
    std::vector<std::string> m_enabled_units;
    std::vector<std::string> m_global_loaded_units;
    std::vector<std::string> m_global_enabled_units;

    auto get_best_locale() const noexcept -> std::string;
    void set_locale(const std::string_view& use_locale) noexcept;
    void set_autostart(const bool& autostart) noexcept;
    auto get_page(const std::string& name) const noexcept -> std::string;
};

#endif  // HELLO_HPP_
