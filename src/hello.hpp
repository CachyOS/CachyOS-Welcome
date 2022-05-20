#ifndef HELLO_HPP_
#define HELLO_HPP_

#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wold-style-cast"
#pragma clang diagnostic ignored "-Wsign-conversion"
#pragma clang diagnostic ignored "-Wdouble-promotion"
#else
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wold-style-cast"
#pragma GCC diagnostic ignored "-Wdouble-promotion"
#pragma GCC diagnostic ignored "-Wuseless-cast"
#pragma GCC diagnostic ignored "-Wsign-conversion"
#endif

#include <gtkmm.h>

#if defined(__clang__)
#pragma clang diagnostic pop
#else
#pragma GCC diagnostic pop
#endif

#include <json.hpp>

class Hello final : public Gtk::Window {
 public:
    Hello(bool is_dev);

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

    auto get_best_locale() const noexcept -> std::string;
    void set_locale(const std::string_view& use_locale) noexcept;
    void set_autostart(const bool& autostart) noexcept;
    auto get_page(const std::string& name) const noexcept -> std::string;
};

#endif  // HELLO_HPP_
