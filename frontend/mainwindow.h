#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include "backend.hpp"
#include "json.hpp"
#include <QComboBox>
#include <QFileDialog>
#include <QMainWindow>
#include <QMessageBox>
#include <QLineEdit>
#include <QPushButton>
#include <QCheckBox>
#include <QVector>
#include <iostream>
#include <string>
#include <map>

using json = nlohmann::json;

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  MainWindow(QWidget *parent = nullptr);
  ~MainWindow();

private:
  void on_comboBox_currentTextChanged(const QString &arg1);
  void on_input_changed(const QString &arg, const std::string &name);
  void on_load_button_clicked();
  void on_save_button_clicked();
  void initialize_ui_variables();
//  void on_text_changed(const QPlainTextEdit * el, const std::string & name);

private:
  Ui::MainWindow *ui;
  json savefile;
  std::string filename;
  QVector<QWidget *> elements;
};
#endif // MAINWINDOW_H
