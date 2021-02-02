using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace frontend
{
    public partial class Form1 : Form
    {
        [DllImport("backend.dll", EntryPoint = "parse_file")]
        public static extern IntPtr parse_file(string t);
        [DllImport("backend.dll", EntryPoint = "free_string")]
        public static extern void free_string(IntPtr t);
        [DllImport("backend.dll", EntryPoint = "write_savegame")]
        public static extern int write_savegame(string original_file, string json);

        public dynamic json;
        public string filename = "..\\..\\..\\..\\backend\\tests\\save_test.sav";

        public Form1()
        {
            IntPtr t = parse_file(filename);

            string parsed = Marshal.PtrToStringAnsi(t);

            free_string(t);

            json = JObject.Parse(parsed);


            System.Diagnostics.Debug.WriteLine($"{filename} {json.difficulty}");
            InitializeComponent();
            InitializeJsonVariables();
        }

        public void InitializeJsonVariables()
        {
            this.kiryu_money.Text = json["kiryu_money"].ToString();
        }

        public void GenerateSavefile()
        {
            string text_json = json.ToString();
            write_savegame(filename, text_json);
        }

        private void button1_Click(object sender, EventArgs e)
        {
            GenerateSavefile();
        }
    }
}
