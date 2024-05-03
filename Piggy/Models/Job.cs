using System.Collections.Generic;

public class Job
{
    public string Name { get; set; } = string.Empty;
    public string SubFolder { get; set; } = string.Empty;
    public string Command { get; set; } = string.Empty;
    public List<string> Arguments { get; set; } = new List<string>();

    public string Display()
    {
        return $"{Name}";
    }

    public string DisplayJobDetail()
    {
        return $"Run: {Command} {string.Join(' ', Arguments)}";
    }
}
