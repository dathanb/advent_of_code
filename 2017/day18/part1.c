#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define SET_OPCODE 0
#define ADD_OPCODE 1
#define MUL_OPCODE 2
#define MOD_OPCODE 4
#define SND_OPCODE 5
#define RCV_OPCODE 6
#define JGZ_OPCODE 7

typedef struct {
  int is_immediate;
  char reg;
  int immediate;
} instruction_arg;

typedef struct {
  int opcode;
  instruction_arg first_arg;
  instruction_arg second_arg;
} instruction;

typedef struct {
  int num_instructions;
  instruction instructions[100];
} program;

typedef struct {
  program code;
  long registers[26];
  int ip; // instruction pointer
  int last_sound;
  int should_break;
} process;

long get_register(process *proc, char reg) {
  return proc->registers[reg-'a'];
}

void set_register(process *proc, char reg, long value) {
  proc->registers[reg-'a'] = value;
}

void init_registers(process *proc) {
  for(char i='a'; i<='z'; i++) {
    set_register(proc, i, 0);
  }
}

void debug_process(process *proc) {
  for(char i='a'; i<='z'; i++) {
    printf("%c: %d\t", i, get_register(proc, i));
  }
  printf("ip: %d\tlast_sound: %d\tshould_break: %d", proc->ip, proc->last_sound, proc->should_break);

  printf("\n");
}

void read_instruction_arg(instruction_arg *arg) {
  if (fscanf(stdin, " %d", &(arg->immediate))) {
    arg->is_immediate = 1;
  } else {
    arg->is_immediate = 0;
    fscanf(stdin, " %c", &(arg->reg));
  }
}

instruction read_instruction() {
  instruction i = {};
  char opcode[4] = {0};
  fscanf(stdin, "%3s", opcode);
  if(strcmp("snd", opcode) == 0) {
    i.opcode = SND_OPCODE;
    read_instruction_arg(&(i.first_arg));
  } else if (strcmp("set", opcode) == 0) {
    i.opcode = SET_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("add", opcode) == 0) {
    i.opcode = ADD_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("mul", opcode) == 0) {
    i.opcode = MUL_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("mod", opcode) == 0) {
    i.opcode = MOD_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("rcv", opcode) == 0) {
    i.opcode = RCV_OPCODE;
    read_instruction_arg(&(i.first_arg));
  } else if (strcmp("jgz", opcode) == 0) {
    i.opcode = JGZ_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  }

  return i;
}

void print_instruction_arg(instruction_arg arg) {
  if(arg.is_immediate) {
    fprintf(stdout, "%d", arg.immediate);
  } else {
    fprintf(stdout, "%c", arg.reg);
  }
}

void print_instruction(instruction i) {
  switch(i.opcode) {
    case SET_OPCODE:
      fprintf(stdout, "set %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case ADD_OPCODE:
      fprintf(stdout, "add %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case MUL_OPCODE:
      fprintf(stdout, "mul %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case MOD_OPCODE:
      fprintf(stdout, "mod %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case SND_OPCODE:
      fprintf(stdout, "snd ");
      print_instruction_arg(i.first_arg);
      fprintf(stdout, "\n");
      break;
    case RCV_OPCODE:
      fprintf(stdout, "rcv ");
      print_instruction_arg(i.first_arg);
      fprintf(stdout, "\n");
      break;
    case JGZ_OPCODE:
      fprintf(stdout, "jgz ");
      print_instruction_arg(i.first_arg);
      fprintf(stdout, " ");
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
  }
}

program load_code() {
  program prog;
  int num_instructions = 0;
  instruction *i = malloc(sizeof(instruction));
  *i = read_instruction();
  while (!feof(stdin)) {
    prog.instructions[num_instructions++] = *i;
    print_instruction(*i);
    *i = read_instruction();
  }
  return prog;
}

void set_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void add_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) + i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) + get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void mul_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) * i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) * get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void mod_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) % i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) % get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void snd_instr(process *p, instruction *i){
  if (i->first_arg.is_immediate) {
    p->last_sound = i->first_arg.immediate;
  } else {
    p->last_sound = get_register(p, i->first_arg.reg);
  }
  p->ip++;
}

void rcv_instr(process *p, instruction *i){
  if(i->first_arg.is_immediate) {
    if(i->first_arg.immediate) {
      p->should_break = 1;
    }
  } else {
    if(get_register(p, i->first_arg.reg)) {
      p->should_break = 1;
    }
  }
  p->ip++;
}

void jgz_instr(process *p, instruction *i){
  if(i->first_arg.is_immediate) {
    if(i->first_arg.immediate <= 0) {
      p->ip++;
      return;
    }
  } else {
    if(get_register(p, i->first_arg.reg) <= 0) {
      p->ip++;
      return;
    }
  }

  int delta = 0;
  if(i->second_arg.is_immediate) { 
    delta = i->second_arg.immediate;
  } else {
    delta = get_register(p, i->second_arg.reg);
  }

  p->ip += delta;
}

void exec_instr(process *p) {
  instruction i = p->code.instructions[p->ip];
}

void init_process(process *p) {
  init_registers(p);
  p->last_sound = 0;
  p->ip = 0;
  p->should_break = 0;
}

void step(process *p) {
  instruction *i = &(p->code.instructions[p->ip]);
  print_instruction(*i);
  switch(i->opcode) {
    case SET_OPCODE:
      set_instr(p, i);
      break;
    case ADD_OPCODE:
      add_instr(p, i);
      break;
    case MUL_OPCODE:
      mul_instr(p, i);
      break;
    case MOD_OPCODE:
      mod_instr(p, i);
      break;
    case SND_OPCODE:
      snd_instr(p, i);
      break;
    case RCV_OPCODE:
      rcv_instr(p, i);
      break;
    case JGZ_OPCODE:
      jgz_instr(p, i);
      break;
  }
}

void run_process(process *p) {
  debug_process(p);
//  for(int i=0; i<12; i++) {
//    step(p);
//    debug_process(p);
//  }
  while (!p->should_break) {
    step(p);
    debug_process(p);
  }
}

void execute(program prog) {
  process p;
  p.code = prog;
  init_process(&p);
  run_process(&p);
  fprintf(stdout, "Last recovered sound: %d", p.last_sound);
}

int main(int num_args, char** args) {
  program prog = load_code();
  execute(prog);
}

