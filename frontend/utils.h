#include <QComboBox>
#include <QWidget>
#include "json.hpp"

#ifndef UTILS_H
#define UTILS_H

using json = nlohmann::json;
namespace utils {

template<class T = json>
void bind_combobox(QWidget *main_obj, QComboBox *obj, T& savefile, const char *key) {
    main_obj->connect(obj, &QComboBox::currentTextChanged, main_obj, [key, &savefile](QString const &s) {
        savefile[key] = s.toStdString();
    });
}

}

#endif // UTILS_H
