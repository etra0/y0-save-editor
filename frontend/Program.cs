using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using System.Windows.Forms;



namespace frontend
{
    
    static class Program
    {

        [DllImport("backend.dll", EntryPoint = "parse_file")]
        public static extern IntPtr parse_file([MarshalAs(UnmanagedType.LPStr)] string t);
        [DllImport("backend.dll", EntryPoint = "free_string")]
        public static extern void free_string(IntPtr t);
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main()
        {
            string test = "..\\backend\\tests\\save_test.sav";
            IntPtr t = parse_file(test);

            string parsed = Marshal.PtrToStringAnsi(t);

            free_string(t);

            System.Diagnostics.Debug.WriteLine($"{test} {parsed}");
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new Form1());
        }
    }
}
