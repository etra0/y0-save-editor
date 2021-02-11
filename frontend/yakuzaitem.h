#ifndef YAKUZAITEM_H
#define YAKUZAITEM_H

#include <QWidget>
#include <QLabel>
#include <QComboBox>
#include <QLineEdit>
#include <memory>

class YakuzaItem : public QWidget
{
    Q_OBJECT
public:
    explicit YakuzaItem(QWidget *parent = nullptr, uint32_t index = 0);

private:
    uint32_t index;
    std::unique_ptr<QLabel> label_item;
    std::unique_ptr<QComboBox> combobox_item;
    std::unique_ptr<QLabel> label_quantity;
    std::unique_ptr<QLineEdit> line_quantity;

signals:

};

#endif // YAKUZAITEM_H
