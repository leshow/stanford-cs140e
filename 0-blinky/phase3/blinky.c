#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0 = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0 = (volatile unsigned *)(GPIO_BASE + 0x28);

static unsigned int const GPIO_MASK = 0b111;
static unsigned int const GPIO_OUT = 0b001;
static unsigned int const GPIO_IN = 0b000;

static void spin_sleep_us(unsigned int us)
{
  for (unsigned int i = 0; i < us * 6; i++)
  {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms)
{
  spin_sleep_us(ms * 1000);
}
static void fsel(unsigned int pin_num)
{
  unsigned int fsel_pos = (pin_num % 10) * 3;
}
// Q: Now, read the documentation for GPFSELn register on pages 91 and 92.
// We write to this register to set up a pin as an output or input.
// Which value to which field in register GPFSEL1 must be written so that GPIO pin 16
// is set as an output?
// A: 0b001 according to the docs
// I think this would work *GPIO_FSEL1 |= 0b001 << 18; but I want to figure out the general case
int main(void)
{
  // STEP 1: Set GPIO Pin 16 as output.
  *GPIO_FSEL1 |= 0b001 << 18;
  // STEP 2: Continuously set and clear GPIO 16.
  while (1)
  {
    *GPIO_SET0 = 1 << 16;
    spin_sleep_us(100);
    *GPIO_SET0 = 1 << 16;
    spin_sleep_us(100);
  }
}
