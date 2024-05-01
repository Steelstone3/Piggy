using System.Collections.Generic;
using Spectre.Console;

public class Prompt : IPrompt
{
    public string FindFile(IFile file)
    {
        return AnsiConsole.Prompt(
            new TextPrompt<string>("Enter [green]Valid[/] Configuration File Path:")
                .Validate(filePath =>
                {
                    if (!file.IsExistingFile(filePath))
                    {
                        return ValidationResult.Error("File [yellow](piggy.json)[/] [red]doesn't exist[/] in that location");
                    }

                    return ValidationResult.Success();
                })
                .ValidationErrorMessage("[red]That's not a valid file path[/]")
            );
    }

    public bool UseDefault()
    {
        return AnsiConsole.Confirm("Use [green]Default[/] Configuration File?");
    }

    public Job JobSelection(List<Job> jobs)
    {
        return AnsiConsole.Prompt(
            new SelectionPrompt<Job>()
                .UseConverter(job => job.Name)
                .PageSize(10)
                .Title("[green]Select[/] Job:?")
                .MoreChoicesText("[grey](Move up and down to reveal more jobs)[/]")
                .AddChoices(jobs));
    }

    public bool ConfirmSelectedJob(Job job)
    {
        return AnsiConsole.Confirm($"[red]Run:[/] {job.DisplayJobDetail()}");
    }

    public void Print(string message)
    {
        AnsiConsole.WriteLine(message);
    }
}

public interface IPrompt
{
    string FindFile(IFile file);
    bool UseDefault();
    Job JobSelection(List<Job> jobs);
    bool ConfirmSelectedJob(Job job);
    void Print(string message);
}