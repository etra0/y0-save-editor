#ifndef ITEMDIALOG_H
#define ITEMDIALOG_H

#include <QDialog>

namespace Ui {
class ItemDialog;
}

class ItemDialog : public QDialog
{
    Q_OBJECT

public:
    explicit ItemDialog(QWidget *parent = nullptr);
    ~ItemDialog();

private:
    Ui::ItemDialog *ui;
};

#endif // ITEMDIALOG_H
