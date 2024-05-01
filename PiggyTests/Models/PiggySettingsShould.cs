using Xunit;

public class PiggySettingsShould
{
    [Fact]
    public void Construct()
    {
        // Given
        PiggySettings piggySettings = new();

        // Then
        Assert.Equal("piggy.json", piggySettings.ConfigurationFile);
    }
}