using System;

public class PiggySettings
{
    public string ProjectFolder { get; set; } = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
    public string ConfigurationFile { get; set; } = "piggy.json";
}