using Xunit;

public class PiggyShould
{
    [Fact]
    public void Construct()
    {
        // Given
        Piggy piggy = new();

        // Then
        Assert.NotNull(piggy.Settings);
        Assert.NotNull(piggy.Jobs);
        Assert.Empty(piggy.Jobs);
    }
}