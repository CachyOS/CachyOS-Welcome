#include "hello.hpp"

int main(int argc, char** argv) {
    auto app = Gtk::Application::create();

    Hello hello(argc, argv);

    // Shows the window and returns when it is closed.
    return app->run(hello);
}
