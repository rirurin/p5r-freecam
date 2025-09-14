namespace p5rpc.freecam.BuildScript;
public struct ColorRGB
{
    public byte R;
    public byte G;
    public byte B;

    public ColorRGB(byte R, byte G, byte B)
    {
        this.R = R;
        this.G = G;
        this.B = B;
    }

    public override string ToString() => $"\x1b[38;2;{R};{G};{B}m";
}

public struct ClearFormat
{
    public override string ToString() => $"\x1b[0m";
}

public struct BoldFormat
{
    public override string ToString() => $"\x1b[1m";
}

public struct ItalicFormat
{
    public override string ToString() => $"\x1b[3m";
}

public struct UnderlineFormat
{
    public override string ToString() => $"\x1b[4m";
}

public struct StrikthroughFormat
{
    public override string ToString() => $"\x1b[9m";
}