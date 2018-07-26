#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_FSEL0 = (volatile unsigned *)(GPIO_BASE);
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

static void fsel_out(unsigned int pin_num)
{
  // each 32-bit wide space fits 10 FSEL, and our mask is size 3
  unsigned int shift = (pin_num % 10) * 3;
  // to get which FSEL(n), integer div by 10 the pin number, as they are in chunks of 10
  unsigned int offset = pin_num / 10;
  *(GPIO_FSEL0 + offset) |= (GPIO_OUT << shift);
  return;
}

static void set_pin(unsigned int pin_num, unsigned int val)
{
  *GPIO_SET0 = val << pin_num;
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
    *GPIO_SET0 |= 1 << 16;
    spin_sleep_us(100);
    *GPIO_CLR0 |= 1 << 16;
    spin_sleep_us(100);
  }
}
