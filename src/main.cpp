#include "hello.hpp"

int main(int argc, char** argv) {
    auto app = Gtk::Application::create();

    const bool is_dev = 1;

    // Shows the window and returns when it is closed.
    return app->make_window_and_run<Hello>(argc, argv, is_dev);
}
