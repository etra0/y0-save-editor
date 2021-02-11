#include "yakuzaitem.h"

YakuzaItem::YakuzaItem(QWidget *parent, uint32_t index) : QWidget(parent)
{
    auto label = "Item " + QString::number(index);
    label_item = std::make_unique<QLabel>(label, parent, Qt::WindowType::Widget);
    label_item->setGeometry(10, 10, 100, 20);

    combobox_item = std::make_unique<QComboBox>(parent);
    combobox_item->setGeometry(10, 30, 100, 20);
}
