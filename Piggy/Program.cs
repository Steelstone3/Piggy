internal static class Program
{
    internal static void Main()
    {
       IRunService runService = new RunService();
       runService.Run();
    }
}
