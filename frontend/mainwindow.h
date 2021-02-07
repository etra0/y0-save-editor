#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include "json.hpp"
#include "backend.hpp"
#include <string>
#include <QFileDialog>
#include <QPushButton>
#include <QMessageBox>
#include <QComboBox>
#include <iostream>

using json = nlohmann::json;

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private:
    void on_comboBox_currentTextChanged(const QString &arg1);
    void on_combobox_changed(const QString &arg, const std::string &name);
    void on_load_button_clicked();
    void initialize_ui_variables();

private:
    Ui::MainWindow *ui;
    json savefile;

};
#endif // MAINWINDOW_H
