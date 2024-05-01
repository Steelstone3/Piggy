public class FileFinder : IFileFinder
{
    private readonly IPrompt prompt;

    public FileFinder(IPrompt prompt)
    {
        this.prompt = prompt;
    }

    public string FindFile(IFile file, string filePath)
    {
        if (prompt.UseDefault())
        {
            if (!file.IsExistingFile(filePath))
            {
                file.CreateDefaultPiggyConfigurationFile();
            }
        }
        else
        {
            filePath = prompt.FindFile(file);
        }

        return filePath;
    }
}

public interface IFileFinder
{
    string FindFile(IFile file, string filePath);
}