#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/string.h> 
#include <linux/init.h>
#include <linux/proc_fs.h> // trae las funciones para crear archivos en /proc
#include <linux/seq_file.h> // trae las funciones para escribir en archivos en /proc
#include <linux/mm.h> // trae las funciones para manejar la memoria
#include <linux/sched.h> // trae las funciones para manejar los procesos
#include <linux/timer.h> // trae las funciones para manejar los timers
#include <linux/jiffies.h> // trae las funciones para manejar los jiffies, que son los ticks del sistema
#include <linux/uaccess.h>
#include <linux/tty.h>
#include <linux/sched/signal.h>
#include <linux/fs.h>        
#include <linux/slab.h>      
#include <linux/sched/mm.h>
#include <linux/binfmts.h>
#include <linux/timekeeping.h>


MODULE_LICENSE("GPL");
MODULE_AUTHOR("Rami");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y CPU");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_202010044" // nombre del archivo en /proc
#define MAX_CMDLINE_LENGTH 256
#define CONTAINER_ID_LENGTH

static char *get_process_cmdline(struct task_struct *task){

    struct mm_struct *mm;
    char *cmdline,*p;
    unsigned long arg_start,arg_end,env_start;
    int i,len;


    cmdline = kmalloc(MAX_CMDLINE_LENGTH,GFP_KERNEL);

    if(!cmdline) return NULL;

    mm = get_task_mm(task);

    if(!mm){
        kfree(cmdline);
        return NULL;
    }

    down_read(&mm->mmap_lock);
    arg_start = mm->arg_start;
    arg_end = mm->arg_end;
    env_start = mm->env_start;
    up_read(&mm->mmap_lock);

    len = arg_end- arg_start;

    if(len>MAX_CMDLINE_LENGTH-1) len=MAX_CMDLINE_LENGTH-1;

    if(access_process_vm(task,arg_start,cmdline,len,0)!=len){
        mmput(mm);
        kfree(cmdline);
        return NULL;
    }


    cmdline[len] = '\0';

    p=cmdline;

    for( i= 0;i<len; i++){
        if(p[i]=='\0') p[i]=' ';
    }

    mmput(mm);
    return cmdline;


}

/* 
    Esta función se encarga de obtener la información de la memoria
    - si_meminfo: recibe un puntero a una estructura sysinfo, la cual se llena con la información de la memoria
*/
static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si; // estructura que contiene la informacion de la memoria
    struct task_struct *task;
    //struct list_head *list;
    unsigned long total_jiffies = jiffies;
    int first_process=1;

    

    si_meminfo(&si); // obtiene la informacion de la memoria

    /*  
        El seq_printf se encarga de escribir en el archivo en /proc
        - m: es el archivo en /pro
    */
    seq_printf(m,"{\n");
    
    seq_printf(m, "\"ram_total\": %lu, \n", si.totalram * 4);
    seq_printf(m, "\"ram_free\": %lu, \n", si.freeram * 4);
    seq_printf(m, "\"ram_usage\": %lu, \n", (si.totalram - si.freeram) * 4);

    seq_printf(m,"\"processes\": [\n");

    for_each_process(task){
        if(strcmp(task->comm,"containerd-shim")==0){
            unsigned long vsz = 0;
            unsigned long rss = 0;
            unsigned long totalram= si.totalram*4;
            unsigned long mem_usage = 0;
            unsigned long cpu_usage = 0;
            char *cmdline = NULL;


            if(task->mm){
                vsz = task-> mm->total_vm << (PAGE_SHIFT - 10);
                rss = get_mm_rss(task->mm) << (PAGE_SHIFT - 10);

                mem_usage= (rss*10000)/ totalram;
            }

            unsigned long total_time= task->utime + task->stime;
            cpu_usage = (total_time * 10000)/ total_jiffies;
            cmdline = get_process_cmdline(task);

            if(!first_process){
                seq_printf(m,",\n");
            }else{
                first_process=0;
                
            }


            seq_printf(m, "  {\n");
            seq_printf(m, "    \"PID\": %d,\n", task->pid);
            seq_printf(m, "    \"Name\": \"%s\",\n", task->comm);
            seq_printf(m, "    \"Cmdline\": \"%s\",\n", cmdline ? cmdline : "N/A");
            seq_printf(m, "    \"Vsz\": %lu,\n", vsz);
            seq_printf(m, "    \"Rss\": %lu,\n", rss);
            seq_printf(m, "    \"MemoryUsage\": %lu.%02lu,\n", mem_usage / 100, mem_usage % 100);
            seq_printf(m, "    \"CPUUsage\": %lu.%02lu\n", cpu_usage / 100, cpu_usage % 100);
            seq_printf(m, "  }");

            if(cmdline){
                kfree(cmdline);
            }

        }


    }

    
 

    

    
    seq_printf(m, "\n]\n}\n");
    return 0;
};

/* 
    Esta función se ejecuta cuando se abre el archivo en /proc
    - single_open: se encarga de abrir el archivo y ejecutar la función sysinfo_show
*/
static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

/* 
    Esta estructura contiene las operaciones a realizar cuando se accede al archivo en /proc
    - proc_open: se ejecuta cuando se abre el archivo
    - proc_read: se ejecuta cuando se lee el archivo
*/

static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};


/* 
    Esta macro se encarga de hacer dos cosas:
    1. Ejecutar la función proc_create, la cual recibe el nombre del archivo a guardar en /proc, permisos,
        y la estructura con las operaciones a realizar

    2. Imprimir un mensaje en el log del kernel
*/
static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "Modulo sysinfo cargado \n");
    return 0;
}

/* 
    Esta macro se encarga de hacer dos cosas:
    1. Ejecutar la función remove_proc_entry, la cual recibe el nombre del archivo a eliminar de /proc
*/
static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "Modulo sysinfo desinstalado\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);